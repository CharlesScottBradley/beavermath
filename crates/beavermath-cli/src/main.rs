//! beavermath CLI: render, filter, doctor, cache, repair.

use beavermath_adapter::grok_session::{
    latest_session_for_cwd, present_last_assistant, write_session_projections,
};
use beavermath_adapter::{
    parse_present_request, present_message_v1, present_to_machine_json, process_hook_json,
    LifecycleStore, PresentationMode,
};
use beavermath_core::diagnostics::{DiagnosticCode, ExitStatus};
use beavermath_core::hash::sha256_hex;
use beavermath_core::repair::{repair_suggest, HeuristicRepairProvider, RepairRequestV1};
use beavermath_core::types::{
    AccessibilityPreference, DisplayMode, OutputContextV1, OutputKind, TrustedCapability,
};
use beavermath_core::{DocumentResultV1, ProcessOptions, SCHEMA_VERSION};
use beavermath_render::{
    process_document_with_visuals, process_expression_with_visuals, PersistentCache,
};
use beavermath_terminal::decision::{
    decide_presentation, graphics_preference_allows, PresentationDecision,
};
use beavermath_terminal::escape::{escape_for_terminal_text, scan_machine_channel};
use beavermath_terminal::present::encode_graphics_for_png;
use clap::{Parser, Subcommand, ValueEnum};
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "beavermath",
    version,
    about = "Source-preserving terminal math renderer"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render an expression or document
    Render {
        /// Expression text (optional if stdin/file used)
        expression: Option<String>,
        /// Read input from file
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = Format::Semantic)]
        format: Format,
        /// Treat input as direct TeX expression
        #[arg(long)]
        direct: bool,
        #[arg(long, value_enum, default_value_t = ModeArg::Inline)]
        mode: ModeArg,
        #[arg(long)]
        png: bool,
        #[arg(long)]
        svg: bool,
        /// Named graphics preference (cannot authorize without trusted capability)
        #[arg(long)]
        graphics: Option<String>,
        /// Explicit local trusted capability: kitty | iterm2
        #[arg(long)]
        trusted_capability: Option<String>,
        #[arg(long)]
        no_cache: bool,
    },
    /// Filter a document, replacing math with semantic text
    Filter {
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Optional inline document
        document: Option<String>,
    },
    /// Report output context and backend decision
    Doctor {
        #[arg(long)]
        trusted_capability: Option<String>,
        #[arg(long)]
        graphics: Option<String>,
    },
    /// Cache management
    Cache {
        #[command(subcommand)]
        action: CacheCmd,
    },
    /// Opt-in ASCII/Unicode repair proposals
    Repair {
        input: String,
        #[arg(long)]
        consent: bool,
        #[arg(long)]
        render: bool,
    },
    /// Machine-safe harness hook: JSON HookRequestV1 on stdin → HookResponseV1 on stdout
    Hook {
        /// Optional file-backed lifecycle store directory
        #[arg(long)]
        store_dir: Option<PathBuf>,
    },
    /// In-chat presentation: exact source → display_text (JSON PresentRequestV1 or raw document on stdin)
    Present {
        /// Optional document (else stdin)
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Inline source when not using file/stdin JSON
        source: Option<String>,
        #[arg(long, value_enum, default_value_t = PresentModeArg::Semantic)]
        mode: PresentModeArg,
    },
    /// Project Grok session assistants through present (never mutates chat_history)
    GrokSession {
        /// Session directory (default: latest for cwd)
        #[arg(long)]
        session_dir: Option<PathBuf>,
        /// Working directory used to find the latest session
        #[arg(long)]
        cwd: Option<PathBuf>,
        /// Write beavermath_display/VIEWPORT.md next to the session
        #[arg(long, default_value_t = true)]
        write_viewport: bool,
        /// Print last assistant presentation JSON to stdout only
        #[arg(long)]
        last_only: bool,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum PresentModeArg {
    Semantic,
    Unicode,
    PixelRef,
}

#[derive(Subcommand, Debug)]
enum CacheCmd {
    Stats {
        #[arg(long, default_value = ".beavermath-cache")]
        path: PathBuf,
    },
    Clear {
        #[arg(long, default_value = ".beavermath-cache")]
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Format {
    Auto,
    Semantic,
    Unicode,
    Ascii,
    Png,
    Svg,
    Json,
    Jsonl,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ModeArg {
    Inline,
    Block,
}

fn print_json(value: &impl serde::Serialize) -> Result<(), ExitStatus> {
    match serde_json::to_string_pretty(value) {
        Ok(s) => {
            println!("{s}");
            Ok(())
        }
        Err(e) => {
            eprintln!("beavermath[json]: {e}");
            Err(ExitStatus::Io)
        }
    }
}

fn print_text_line(s: &str) {
    if s.ends_with('\n') {
        print!("{s}");
    } else {
        println!("{s}");
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(status) => ExitCode::from(status.code() as u8),
        Err(status) => ExitCode::from(status.code() as u8),
    }
}

fn run(cli: Cli) -> Result<ExitStatus, ExitStatus> {
    match cli.command {
        Commands::Render {
            expression,
            file,
            format,
            direct,
            mode,
            png,
            svg,
            graphics,
            trusted_capability,
            no_cache: _,
        } => {
            let input = read_input(expression, file)?;
            let ctx = build_context(trusted_capability.as_deref());
            let stdout_is_tty = io::stdout().is_terminal();
            let mut ctx = ctx;
            // Reflect real stdout TTY into context (build_context already does; keep explicit)
            ctx.is_direct_tty = stdout_is_tty && ctx.is_direct_tty;
            if !stdout_is_tty {
                ctx.is_direct_tty = false;
                ctx.output_kind = OutputKind::Pipe;
            }

            let decision = resolve_decision(format, graphics.as_deref(), &ctx);
            let opts = ProcessOptions {
                request_png: png
                    || matches!(format, Format::Png)
                    || matches!(
                        decision,
                        PresentationDecision::KittyPng | PresentationDecision::Iterm2Png
                    ),
                request_svg: svg || matches!(format, Format::Svg),
                ..ProcessOptions::default() // auto_protect = true
            };

            if direct {
                let m = match mode {
                    ModeArg::Inline => DisplayMode::Inline,
                    ModeArg::Block => DisplayMode::Block,
                };
                let art =
                    process_expression_with_visuals(input.as_bytes(), m, &opts).map_err(|d| {
                        eprintln!("{}", d.human_line());
                        ExitStatus::ParseOrRender
                    })?;
                emit_artifact_format(&art, format, &ctx, decision, graphics.as_deref())?;
            } else {
                // Empty host ranges; process_document merges default URL/code protection.
                let doc =
                    process_document_with_visuals(input.as_bytes(), &[], &opts).map_err(|d| {
                        eprintln!("{}", d.human_line());
                        ExitStatus::ParseOrRender
                    })?;
                emit_document_format(&doc, format, &ctx, decision, graphics.as_deref())?;
            }
            Ok(ExitStatus::Success)
        }
        Commands::Filter { file, document } => {
            let input = read_input(document, file)?;
            let doc =
                process_document_with_visuals(input.as_bytes(), &[], &ProcessOptions::default())
                    .map_err(|d| {
                        eprintln!("{}", d.human_line());
                        ExitStatus::ParseOrRender
                    })?;
            let text = doc.filtered_text.unwrap_or(input);
            let safe = escape_for_terminal_text(&text);
            let scan = scan_machine_channel(safe.as_bytes());
            if !scan.clean {
                eprintln!(
                    "beavermath[{}]: internal contamination detected",
                    DiagnosticCode::Internal.as_str()
                );
                return Err(ExitStatus::Internal);
            }
            print_text_line(&safe);
            let _ = io::stdout().flush();
            Ok(ExitStatus::Success)
        }
        Commands::Doctor {
            trusted_capability,
            graphics,
        } => {
            let mut ctx = build_context(trusted_capability.as_deref());
            let stdout_is_tty = io::stdout().is_terminal();
            if !stdout_is_tty {
                ctx.is_direct_tty = false;
                ctx.output_kind = OutputKind::Pipe;
            }
            let (dec, reason) = decide_presentation(&ctx);
            let pref = graphics_preference_allows(graphics.as_deref(), &ctx);
            let report = serde_json::json!({
                "schema_version": SCHEMA_VERSION,
                "beavermath_version": env!("CARGO_PKG_VERSION"),
                "stdout_is_terminal": stdout_is_tty,
                "output_context": ctx,
                "presentation": format!("{dec:?}"),
                "graphics_preference_decision": format!("{pref:?}"),
                "decision_reason": reason,
                "graphics_preference": graphics,
                "v1_terminal_queries": 0,
                "auto_protect": true,
                "limits": beavermath_core::LimitsV1::default(),
            });
            print_json(&report)?;
            Ok(ExitStatus::Success)
        }
        Commands::Cache { action } => match action {
            CacheCmd::Stats { path } => {
                let cache = if path.exists() {
                    PersistentCache::open(&path, "local")
                        .unwrap_or_else(|_| PersistentCache::disabled())
                } else {
                    PersistentCache::disabled()
                };
                print_json(&cache.stats())?;
                Ok(ExitStatus::Success)
            }
            CacheCmd::Clear { path } => {
                if path.exists() {
                    let cache =
                        PersistentCache::open(&path, "local").map_err(|_| ExitStatus::Io)?;
                    cache.clear().map_err(|_| ExitStatus::Io)?;
                }
                println!("{{\"cleared\":true}}");
                Ok(ExitStatus::Success)
            }
        },
        Commands::Repair {
            input,
            consent,
            render,
        } => {
            let provider = HeuristicRepairProvider;
            let req = RepairRequestV1 {
                original: input.clone(),
                consent,
            };
            match repair_suggest(&req, &provider) {
                Ok(prop) => {
                    if render {
                        let opts = ProcessOptions::default();
                        match process_expression_with_visuals(
                            prop.inferred_tex.as_bytes(),
                            DisplayMode::Inline,
                            &opts,
                        ) {
                            Ok(art) => {
                                let out = serde_json::json!({
                                    "proposal": prop,
                                    "rendered_semantic": art.semantic.text,
                                    "original_sha256": prop.original_sha256,
                                });
                                print_json(&out)?;
                            }
                            Err(d) => {
                                eprintln!("{}", d.human_line());
                                return Err(ExitStatus::ParseOrRender);
                            }
                        }
                    } else {
                        print_json(&prop)?;
                    }
                    if prop.original_sha256 != sha256_hex(input.as_bytes()) {
                        eprintln!("beavermath[repair]: original digest mismatch");
                        return Err(ExitStatus::Io);
                    }
                    Ok(ExitStatus::Success)
                }
                Err(d) => {
                    eprintln!("{}", d.human_line());
                    Err(ExitStatus::Usage)
                }
            }
        }
        Commands::Hook { store_dir } => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).map_err(|e| {
                eprintln!("beavermath[io]: {e}");
                ExitStatus::Io
            })?;
            let mut store = if let Some(dir) = store_dir {
                LifecycleStore::with_persist_dir(dir).map_err(|e| {
                    eprintln!("beavermath[io]: store: {e}");
                    ExitStatus::Io
                })?
            } else {
                LifecycleStore::new()
            };
            match process_hook_json(&mut store, &buf) {
                Ok(json) => {
                    // Machine channel only
                    let scan = scan_machine_channel(json.as_bytes());
                    if !scan.clean {
                        eprintln!(
                            "beavermath[{}]: refused ESC in hook stdout",
                            DiagnosticCode::MachineChannel.as_str()
                        );
                        return Err(ExitStatus::Internal);
                    }
                    print!("{json}");
                    let _ = io::stdout().flush();
                    Ok(ExitStatus::Success)
                }
                Err((status, msg)) => {
                    eprintln!("beavermath[hook]: {msg}");
                    Err(status)
                }
            }
        }
        Commands::GrokSession {
            session_dir,
            cwd,
            write_viewport,
            last_only,
        } => {
            let session = if let Some(d) = session_dir {
                d
            } else {
                let cwd = cwd.unwrap_or_else(|| {
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                });
                let cwd_s = cwd.to_string_lossy().to_string();
                latest_session_for_cwd(&cwd_s).ok_or_else(|| {
                    eprintln!(
                        "beavermath[grok-session]: no session under GROK_HOME for cwd {cwd_s}"
                    );
                    ExitStatus::Io
                })?
            };
            if !session.join("chat_history.jsonl").is_file() {
                eprintln!(
                    "beavermath[grok-session]: missing chat_history.jsonl in {}",
                    session.display()
                );
                return Err(ExitStatus::Io);
            }
            if last_only {
                match present_last_assistant(&session) {
                    Ok(Some(p)) => {
                        let json = present_to_machine_json(&p).map_err(|e| {
                            eprintln!("beavermath[present]: {e}");
                            ExitStatus::Internal
                        })?;
                        print!("{json}");
                        return Ok(ExitStatus::Success);
                    }
                    Ok(None) => {
                        eprintln!("beavermath[grok-session]: no assistant messages");
                        return Err(ExitStatus::InvalidInput);
                    }
                    Err(e) => {
                        eprintln!("beavermath[grok-session]: {e}");
                        return Err(ExitStatus::ParseOrRender);
                    }
                }
            }
            if write_viewport {
                match write_session_projections(&session) {
                    Ok(view) => {
                        eprintln!(
                            "beavermath[grok-session]: wrote display projections (source untouched)"
                        );
                        println!("{}", view.display());
                        Ok(ExitStatus::Success)
                    }
                    Err(e) => {
                        eprintln!("beavermath[grok-session]: {e}");
                        Err(ExitStatus::Io)
                    }
                }
            } else {
                match present_last_assistant(&session) {
                    Ok(Some(p)) => {
                        println!("{}", p.display_text);
                        Ok(ExitStatus::Success)
                    }
                    Ok(None) => Err(ExitStatus::InvalidInput),
                    Err(e) => {
                        eprintln!("beavermath[grok-session]: {e}");
                        Err(ExitStatus::ParseOrRender)
                    }
                }
            }
        }
        Commands::Present { file, source, mode } => {
            let input = if let Some(p) = file {
                std::fs::read_to_string(p).map_err(|e| {
                    eprintln!("beavermath[io]: {e}");
                    ExitStatus::Io
                })?
            } else if let Some(s) = source {
                s
            } else {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf).map_err(|e| {
                    eprintln!("beavermath[io]: {e}");
                    ExitStatus::Io
                })?;
                buf
            };
            let mut req = parse_present_request(&input).map_err(|e| {
                eprintln!("beavermath[present]: {e}");
                ExitStatus::Usage
            })?;
            req.mode = Some(match mode {
                PresentModeArg::Semantic => PresentationMode::Semantic,
                PresentModeArg::Unicode => PresentationMode::Unicode,
                PresentModeArg::PixelRef => PresentationMode::PixelRef,
            });
            match present_message_v1(&req) {
                Ok(p) => match present_to_machine_json(&p) {
                    Ok(json) => {
                        print!("{json}");
                        let _ = io::stdout().flush();
                        Ok(ExitStatus::Success)
                    }
                    Err(e) => {
                        eprintln!("beavermath[present]: {e}");
                        Err(ExitStatus::Internal)
                    }
                },
                Err(e) => {
                    eprintln!("beavermath[present]: {e}");
                    Err(ExitStatus::ParseOrRender)
                }
            }
        }
    }
}

fn resolve_decision(
    format: Format,
    graphics: Option<&str>,
    ctx: &OutputContextV1,
) -> PresentationDecision {
    match format {
        Format::Semantic | Format::Json | Format::Jsonl | Format::Ascii | Format::Unicode => {
            PresentationDecision::SemanticOnly
        }
        Format::Png | Format::Svg => PresentationDecision::SemanticOnly, // explicit artifacts, not terminal protocols
        Format::Auto => {
            if graphics.is_some() {
                graphics_preference_allows(graphics, ctx)
            } else {
                decide_presentation(ctx).0
            }
        }
    }
}

fn build_context(trusted: Option<&str>) -> OutputContextV1 {
    let is_tty = io::stdout().is_terminal();
    let mut ctx = OutputContextV1 {
        output_kind: if is_tty {
            OutputKind::Tty
        } else {
            OutputKind::Pipe
        },
        is_direct_tty: is_tty,
        accessibility: AccessibilityPreference::Default,
        trusted_capability: TrustedCapability::None,
        ..Default::default()
    };
    match trusted {
        Some("kitty") => ctx.trusted_capability = TrustedCapability::KittyDirect,
        Some("iterm2") => ctx.trusted_capability = TrustedCapability::Iterm2Inline,
        _ => {}
    }
    if !is_tty {
        ctx.is_direct_tty = false;
        ctx.output_kind = OutputKind::Pipe;
    }
    ctx
}

fn read_input(expression: Option<String>, file: Option<PathBuf>) -> Result<String, ExitStatus> {
    if let Some(p) = file {
        return std::fs::read_to_string(p).map_err(|e| {
            eprintln!("beavermath[io]: {e}");
            ExitStatus::Io
        });
    }
    if let Some(e) = expression {
        return Ok(e);
    }
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).map_err(|e| {
        eprintln!("beavermath[io]: {e}");
        ExitStatus::Io
    })?;
    Ok(buf)
}

fn emit_document_format(
    doc: &DocumentResultV1,
    format: Format,
    ctx: &OutputContextV1,
    decision: PresentationDecision,
    _graphics: Option<&str>,
) -> Result<(), ExitStatus> {
    match format {
        Format::Json => {
            let s = serde_json::to_string_pretty(doc).map_err(|_| ExitStatus::Internal)?;
            write_machine(s.as_bytes())?;
        }
        Format::Jsonl => {
            for e in &doc.expressions {
                let s = serde_json::to_string(e).map_err(|_| ExitStatus::Internal)?;
                write_machine(format!("{s}\n").as_bytes())?;
            }
            if doc.expressions.is_empty() {
                write_machine(
                    format!(
                        "{}\n",
                        serde_json::json!({"status":"unchanged","input_sha256": doc.input_sha256})
                    )
                    .as_bytes(),
                )?;
            }
        }
        Format::Semantic => {
            write_semantic_document(doc)?;
        }
        Format::Auto => {
            // Universal artifact first: semantic/filter text is always written.
            write_semantic_document(doc)?;
            // Then optional typed pixel frames when decision + PNG artifacts allow.
            emit_optional_graphics(doc, decision)?;
            let _ = ctx;
        }
        Format::Unicode => {
            let mut out = String::new();
            if doc.expressions.is_empty() {
                out = doc.input_exact_utf8.clone();
            } else if matches!(decision, PresentationDecision::VisualUnicode)
                || matches!(format, Format::Unicode)
            {
                // Rebuild with unicode overlays where available via filtered semantic path:
                // Prefer per-expression unicode in a simple join for unicode format.
                let mut cursor = 0usize;
                let input = doc.input_exact_utf8.as_str();
                for e in &doc.expressions {
                    let s = e.expression_byte_range.start;
                    let end = e.expression_byte_range.end;
                    out.push_str(&input[cursor..s]);
                    if let Some(u) = &e.visual.unicode {
                        out.push_str(u);
                    } else {
                        out.push_str(&e.semantic.text);
                    }
                    cursor = end;
                }
                out.push_str(&input[cursor..]);
            }
            write_machine(escape_for_terminal_text(&out).as_bytes())?;
        }
        Format::Ascii => {
            let mut out = String::new();
            let mut cursor = 0usize;
            let input = doc.input_exact_utf8.as_str();
            for e in &doc.expressions {
                let s = e.expression_byte_range.start;
                let end = e.expression_byte_range.end;
                out.push_str(&input[cursor..s]);
                if let Some(a) = &e.visual.ascii {
                    out.push_str(a);
                } else {
                    out.push_str(&e.semantic.text);
                }
                cursor = end;
            }
            out.push_str(&input[cursor..]);
            if out.is_empty() {
                out = doc.input_exact_utf8.clone();
            }
            let ascii: String = out.chars().filter(|c| c.is_ascii()).collect();
            write_machine(ascii.as_bytes())?;
        }
        Format::Svg => {
            for e in &doc.expressions {
                if let Some(svg) = &e.visual.svg {
                    let raw = decode_b64_approx(&svg.bytes_b64);
                    write_machine(&raw)?;
                }
            }
        }
        Format::Png => {
            for e in &doc.expressions {
                if let Some(png) = &e.visual.png {
                    let raw = decode_b64_approx(&png.bytes_b64);
                    io::stdout().write_all(&raw).map_err(|_| ExitStatus::Io)?;
                }
            }
        }
    }
    Ok(())
}

fn write_semantic_document(doc: &DocumentResultV1) -> Result<(), ExitStatus> {
    let text = if doc.expressions.is_empty() {
        doc.input_exact_utf8.clone()
    } else {
        doc.filtered_text
            .clone()
            .unwrap_or_else(|| doc.input_exact_utf8.clone())
    };
    let safe = escape_for_terminal_text(&text);
    write_machine(safe.as_bytes())
}

fn emit_optional_graphics(
    doc: &DocumentResultV1,
    decision: PresentationDecision,
) -> Result<(), ExitStatus> {
    if !matches!(
        decision,
        PresentationDecision::KittyPng | PresentationDecision::Iterm2Png
    ) {
        return Ok(());
    }
    let mut image_id = 1u32;
    for e in &doc.expressions {
        let Some(png_art) = &e.visual.png else {
            continue;
        };
        let png = decode_b64_approx(&png_art.bytes_b64);
        match encode_graphics_for_png(&png, decision, image_id) {
            Ok(Some(frame)) => {
                // Typed protocol path: intentional ESC frames only after semantic text.
                write_protocol(frame.bytes.as_slice())?;
                image_id = image_id.wrapping_add(1).max(1);
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!(
                    "beavermath[{}]: graphics encode failed: {err}",
                    DiagnosticCode::RenderFailed.as_str()
                );
                // Visual failure must not remove semantic (already written).
            }
        }
    }
    Ok(())
}

fn emit_artifact_format(
    art: &beavermath_core::MathArtifactV1,
    format: Format,
    ctx: &OutputContextV1,
    decision: PresentationDecision,
    graphics: Option<&str>,
) -> Result<(), ExitStatus> {
    let doc = DocumentResultV1 {
        schema_version: SCHEMA_VERSION.into(),
        status: beavermath_core::RenderStatusV1::Ok,
        input_exact_utf8: art.input_exact_utf8.clone(),
        input_sha256: art.input_sha256.clone(),
        partition_ok: true,
        expressions: vec![art.clone()],
        diagnostics: vec![],
        filtered_text: Some(art.semantic.text.clone()),
    };
    emit_document_format(&doc, format, ctx, decision, graphics)
}

fn write_machine(bytes: &[u8]) -> Result<(), ExitStatus> {
    let scan = scan_machine_channel(bytes);
    if !scan.clean {
        if bytes.windows(4).any(|w| w == b"\x89PNG") {
            // binary PNG format path
        } else {
            eprintln!(
                "beavermath[{}]: refused to write terminal controls to machine channel",
                DiagnosticCode::MachineChannel.as_str()
            );
            return Err(ExitStatus::Internal);
        }
    }
    let mut out = io::stdout().lock();
    out.write_all(bytes).map_err(|_| ExitStatus::Io)?;
    Ok(())
}

/// Protocol frames (Kitty/iTerm2) are only written on the authorized TTY pixel path.
fn write_protocol(bytes: &[u8]) -> Result<(), ExitStatus> {
    let mut out = io::stdout().lock();
    out.write_all(bytes).map_err(|_| ExitStatus::Io)?;
    let _ = out.flush();
    Ok(())
}

fn decode_b64_approx(s: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 3 < bytes.len() {
        let (a, b, c, d) = (bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]);
        i += 4;
        let va = val(a).unwrap_or(0) as u32;
        let vb = val(b).unwrap_or(0) as u32;
        let vc = if c == b'=' {
            0
        } else {
            val(c).unwrap_or(0) as u32
        };
        let vd = if d == b'=' {
            0
        } else {
            val(d).unwrap_or(0) as u32
        };
        let triple = (va << 18) | (vb << 12) | (vc << 6) | vd;
        out.push(((triple >> 16) & 0xff) as u8);
        if c != b'=' {
            out.push(((triple >> 8) & 0xff) as u8);
        }
        if d != b'=' {
            out.push((triple & 0xff) as u8);
        }
    }
    out
}
