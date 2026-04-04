//! Base HTML wrappers and CSS handling

use crate::helpers::html_escape;

use super::HASH;

/// Minimal style struct for base HTML rendering
pub struct BaseStyle {
    pub primary_color: String,
    pub text_color: String,
    pub background_color: String,
    pub border_radius: String,
    pub font_family: String,
}

impl Default for BaseStyle {
    fn default() -> Self {
        Self {
            primary_color: String::from("#F38020"),
            text_color: String::from("#333333"),
            background_color: String::from("#ffffff"),
            border_radius: String::from("4px"),
            font_family: String::from("system-ui, -apple-system, sans-serif"),
        }
    }
}

/// Base HTML wrapper for admin pages
pub fn base_html(title: &str, content: &str, style: &BaseStyle) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en"
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link rel="icon" type="image/svg+xml" href="/logo.svg">
    <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32.png">
    <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16.png">
    <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
    <link rel="manifest" href="/site.webmanifest">
    <meta name="msapplication-TileColor" content="{hash}1A1A2E">
    <meta name="theme-color" content="{primary}">
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <style>
        :root {{
            --cal-primary: {primary};
            --cal-primary-text: {hash}C55A11;
            --cal-text: {text};
            --cal-bg: {bg};
            --cal-border-radius: {radius};
            --cal-font: {font};
            --bg-card: {hash}fff;
            --bg-muted: {hash}f5f6f8;
            --border: {hash}e0e0e0;
            --text-muted: {hash}555;
            --success-bg: {hash}d4edda;
            --success-text: {hash}155724;
            --error-bg: {hash}f8d7da;
            --error-text: {hash}721c24;
            --code-bg: {hash}f0f1f3;
            --shadow: 0 1px 3px rgba(0,0,0,.08), 0 1px 2px rgba(0,0,0,.06);
            --shadow-md: 0 4px 6px rgba(0,0,0,.07), 0 2px 4px rgba(0,0,0,.05);
            --focus-ring: 2px solid {hash}C55A11;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --cal-bg: {hash}121212;
                --cal-text: {hash}e0e0e0;
                --cal-primary: {hash}F9A825;
                --cal-primary-text: {hash}F9A825;
                --bg-card: {hash}1e1e1e;
                --bg-muted: {hash}2a2a2a;
                --border: {hash}3a3a3a;
                --text-muted: {hash}aaa;
                --success-bg: {hash}1e3a2f;
                --success-text: {hash}6fcf97;
                --error-bg: {hash}3a1e1e;
                --error-text: {hash}f5a5a5;
                --code-bg: {hash}2a2a2a;
                --shadow: 0 1px 3px rgba(0,0,0,.2);
                --shadow-md: 0 4px 6px rgba(0,0,0,.2);
                --focus-ring: 2px solid {hash}F9A825;
            }}
        }}
        *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: var(--cal-font);
            color: var(--cal-text);
            background: var(--cal-bg);
            line-height: 1.6;
            -webkit-font-smoothing: antialiased;
        }}
        .skip-link {{
            position: absolute;
            top: -100%;
            left: 1rem;
            padding: 0.5rem 1rem;
            background: var(--cal-primary);
            color: {hash}fff;
            border-radius: var(--cal-border-radius);
            z-index: 1000;
            font-weight: 600;
            text-decoration: none;
        }}
        .skip-link:focus {{ top: 0.5rem; }}
        .container {{ max-width: 960px; margin: 0 auto; padding: 1.5rem 1rem; }}
        a {{ color: var(--cal-primary-text); text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        a:focus-visible {{ outline: var(--focus-ring); outline-offset: 2px; border-radius: 2px; }}
        .btn {{
            display: inline-flex;
            align-items: center;
            justify-content: center;
            gap: 0.4rem;
            padding: 0.6rem 1.2rem;
            min-height: 44px;
            background: var(--cal-primary);
            color: {hash}fff;
            border: none;
            border-radius: var(--cal-border-radius);
            cursor: pointer;
            text-decoration: none;
            font-size: 0.875rem;
            font-weight: 500;
            font-family: inherit;
            line-height: 1.2;
            transition: transform 0.1s, box-shadow 0.15s, opacity 0.15s;
        }}
        .btn:hover {{ transform: translateY(-1px); box-shadow: var(--shadow-md); text-decoration: none; }}
        .btn:active {{ transform: translateY(0); }}
        .btn:focus-visible {{ outline: var(--focus-ring); outline-offset: 2px; }}
        .btn:disabled {{ opacity: 0.5; cursor: not-allowed; transform: none; box-shadow: none; }}
        .btn-secondary {{ background: {hash}1A1A2E; }}
        .btn-danger {{ background: {hash}dc3545; }}
        .btn-sm {{
            padding: 0.4rem 0.75rem;
            min-height: 36px;
            font-size: 0.8rem;
        }}
        .form-group {{ margin-bottom: 1.25rem; }}
        .form-group label {{
            display: block;
            margin-bottom: 0.35rem;
            font-weight: 500;
            font-size: 0.9rem;
        }}
        .form-group input,
        .form-group select,
        .form-group textarea {{
            width: 100%;
            padding: 0.6rem 0.75rem;
            min-height: 44px;
            border: 1px solid var(--border);
            border-radius: var(--cal-border-radius);
            font-size: 1rem;
            font-family: inherit;
            background: var(--bg-card);
            color: var(--cal-text);
            transition: border-color 0.15s, box-shadow 0.15s;
        }}
        .form-group input:focus-visible,
        .form-group select:focus-visible,
        .form-group textarea:focus-visible {{
            outline: none;
            border-color: var(--cal-primary);
            box-shadow: 0 0 0 3px rgba(243, 128, 32, 0.15);
        }}
        .form-group input:disabled,
        .form-group select:disabled,
        .form-group textarea:disabled {{
            opacity: 0.6;
            cursor: not-allowed;
            background: var(--bg-muted);
        }}
        .form-group input[type="color"] {{
            padding: 0.25rem;
            height: 44px;
            cursor: pointer;
        }}
        .card {{
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1.25rem;
            margin-bottom: 1.25rem;
            box-shadow: var(--shadow);
        }}
        .success {{
            background: var(--success-bg);
            color: var(--success-text);
            padding: 0.75rem 1rem;
            border-radius: var(--cal-border-radius);
            margin-bottom: 1rem;
            font-weight: 500;
        }}
        .error {{
            background: var(--error-bg);
            color: var(--error-text);
            padding: 0.75rem 1rem;
            border-radius: var(--cal-border-radius);
            margin-bottom: 1rem;
            font-weight: 500;
        }}
        .htmx-indicator {{ display: none; }}
        .htmx-request .htmx-indicator {{ display: inline; }}
        .table-wrap {{
            overflow-x: auto;
            -webkit-overflow-scrolling: touch;
            margin: 0 -0.25rem;
            padding: 0 0.25rem;
        }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{
            padding: 0.6rem 0.75rem;
            text-align: left;
            border-bottom: 1px solid var(--border);
            white-space: nowrap;
        }}
        th {{
            background: var(--bg-muted);
            font-weight: 600;
            font-size: 0.85rem;
            text-transform: uppercase;
            letter-spacing: 0.03em;
            color: var(--text-muted);
        }}
        td {{ font-size: 0.9rem; }}
        code {{
            background: var(--code-bg);
            padding: 0.15rem 0.4rem;
            border-radius: 3px;
            font-family: ui-monospace, 'Cascadia Code', 'Fira Code', monospace;
            font-size: 0.85em;
        }}
        small, .text-muted {{ color: var(--text-muted); }}
        .btn-copy {{
            padding: 0.3rem 0.6rem;
            font-size: 0.75rem;
            background: {hash}1A1A2E;
            white-space: nowrap;
            min-height: 32px;
        }}
        .btn-copy.copied {{ background: {hash}16a34a; }}
        h1 {{ font-size: 1.5rem; font-weight: 700; letter-spacing: -0.01em; }}
        h2 {{ font-size: 1.15rem; font-weight: 600; margin-bottom: 0.5rem; }}
        h3 {{ font-size: 1rem; font-weight: 600; }}
        @media (max-width: 768px) {{
            .container {{ padding: 1rem 0.75rem; }}
            th, td {{ padding: 0.5rem; font-size: 0.85rem; }}
            h1 {{ font-size: 1.3rem; }}
        }}
        @media (max-width: 480px) {{
            .container {{ padding: 0.75rem 0.5rem; }}
            .card {{ padding: 1rem; }}
            .btn {{ padding: 0.5rem 1rem; font-size: 0.8rem; }}
        }}
    </style>
</head>
<body>
    <a href="#main-content" class="skip-link">Skip to main content</a>
    <div class="container" id="main-content" role="main">
        {content}
    </div>
    <nav aria-label="Footer" style="text-align: center; padding: 1.5rem 1rem; color: var(--text-muted); font-size: 0.8rem;">
        <a href="https://ananthb.github.io/concierge-worker/" style="color: var(--text-muted);">Docs</a> &middot;
        <a href="/terms" style="color: var(--text-muted);">Terms</a> &middot;
        <a href="/privacy" style="color: var(--text-muted);">Privacy</a>
        <p style="margin-top: 0.25rem; font-size: 0.7rem;">AGPL-3.0 &mdash; <a href="https://github.com/ananthb/concierge-worker" style="color: var(--text-muted);">source on GitHub</a></p>
    </nav>
    <script>
        function copyUrl(btn, url) {{
            navigator.clipboard.writeText(url).then(function() {{
                btn.textContent = 'Copied!';
                btn.classList.add('copied');
                setTimeout(function() {{
                    btn.textContent = 'Copy';
                    btn.classList.remove('copied');
                }}, 2000);
            }});
        }}
    </script>
</body>
</html>"##,
        title = html_escape(title),
        content = content,
        primary = html_escape(&style.primary_color),
        text = html_escape(&style.text_color),
        bg = html_escape(&style.background_color),
        radius = html_escape(&style.border_radius),
        font = html_escape(&style.font_family),
        hash = HASH,
    )
}
