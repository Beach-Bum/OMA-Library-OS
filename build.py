#!/usr/bin/env python3
"""Build static HTML site from MDX content files."""

import re
import os
import html

CONTENT_DIR = "_content-backup"
OUT_DIR = "site/docs"

PAGES = [
    ("index", "The Manual"),
    ("quickstart", "Getting Started"),
    ("navigation", "On Moving"),
    ("reading", "On Reading"),
    ("registers", "The Three Registers"),
    ("writing", "On Writing"),
    ("lambda", "The \u039b Language"),
    ("journal", "The Journal"),
    ("living-documents", "Living Documents"),
    ("fundament", "The Fundament"),
    ("connecting", "Connecting Libraries"),
    ("commands", "Command Reference"),
    ("impossible-things", "Impossible Things"),
]


def parse_frontmatter(text):
    """Return (meta_dict, body). Frontmatter is simple `key: value` lines."""
    meta = {}
    if text.startswith("---"):
        end = text.find("---", 3)
        if end != -1:
            for line in text[3:end].strip().splitlines():
                if ": " in line:
                    k, v = line.split(": ", 1)
                    meta[k.strip()] = v.strip()
            return meta, text[end + 3:].lstrip("\n")
    return meta, text


def md_to_html(md):
    """Minimal markdown to HTML. Handles what we need."""
    lines = md.split("\n")
    out = []
    in_code = False
    in_table = False
    in_ul = False
    in_ol = False
    table_header_done = False

    i = 0
    while i < len(lines):
        line = lines[i]

        # Code blocks
        if line.startswith("```"):
            if in_code:
                out.append("</code></pre>")
                in_code = False
            else:
                out.append("<pre><code>")
                in_code = True
            i += 1
            continue

        if in_code:
            out.append(html.escape(line))
            i += 1
            continue

        # Close lists if needed
        if in_ul and not line.startswith("- ") and not (line.startswith("  ") and lines[i-1].startswith("- ")):
            out.append("</ul>")
            in_ul = False

        # Tables
        if "|" in line and line.strip().startswith("|"):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            # Check if separator row
            if all(re.match(r'^[-:]+$', c) for c in cells):
                table_header_done = True
                i += 1
                continue
            if not in_table:
                out.append("<table>")
                in_table = True
                tag = "th"
                table_header_done = False
            else:
                tag = "td" if table_header_done else "th"

            row = "<tr>" + "".join(f"<{tag}>{inline(c)}</{tag}>" for c in cells) + "</tr>"
            out.append(row)
            i += 1
            continue
        elif in_table:
            out.append("</table>")
            in_table = False
            table_header_done = False

        # Headings
        m = re.match(r'^(#{1,3})\s+(.*)', line)
        if m:
            level = len(m.group(1))
            text = inline(m.group(2))
            out.append(f"<h{level}>{text}</h{level}>")
            i += 1
            continue

        # Horizontal rule
        if line.strip() == "---":
            out.append("<hr>")
            i += 1
            continue

        # Unordered list
        if line.startswith("- "):
            if not in_ul:
                out.append("<ul>")
                in_ul = True
            out.append(f"<li>{inline(line[2:])}</li>")
            i += 1
            continue

        # Empty line
        if not line.strip():
            i += 1
            continue

        # Paragraph
        para = []
        while i < len(lines) and lines[i].strip() and not lines[i].startswith("#") and not lines[i].startswith("```") and not lines[i].startswith("- ") and not ("|" in lines[i] and lines[i].strip().startswith("|")):
            para.append(lines[i])
            i += 1
        out.append(f"<p>{inline(' '.join(para))}</p>")
        continue

        i += 1

    # Close open tags
    if in_code:
        out.append("</code></pre>")
    if in_table:
        out.append("</table>")
    if in_ul:
        out.append("</ul>")

    return "\n".join(out)


def inline(text):
    """Handle inline markdown."""
    text = html.escape(text)
    # Bold
    text = re.sub(r'\*\*(.+?)\*\*', r'<strong>\1</strong>', text)
    # Italic
    text = re.sub(r'\*(.+?)\*', r'<em>\1</em>', text)
    # Inline code
    text = re.sub(r'`(.+?)`', r'<code>\1</code>', text)
    # Links
    text = re.sub(r'\[(.+?)\]\((.+?)\)', r'<a href="\2">\1</a>', text)
    # &apos;
    text = text.replace("&amp;apos;", "'")
    return text


def nav_html(current_slug):
    items = []
    for slug, title in PAGES:
        href = f"/docs/{slug}.html" if slug != "index" else "/docs/"
        cls = ' class="active"' if slug == current_slug else ""
        items.append(f'<li><a href="{href}"{cls}>{title}</a></li>')
    return "\n".join(items)


def doc_template(title, content_html, current_slug, description, out_name):
    full_title = f"{title} — ΦΜΛ"
    url = f"https://oma.ooo/docs/{out_name}"
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{full_title}</title>
<meta name="description" content="{description}">
<link rel="canonical" href="{url}">
<meta property="og:title" content="{full_title}">
<meta property="og:description" content="{description}">
<meta property="og:type" content="website">
<meta property="og:url" content="{url}">
<meta property="og:site_name" content="ΦΜΛ — Open Manual Archive">
<meta name="twitter:card" content="summary">
<meta name="twitter:title" content="{full_title}">
<meta name="twitter:description" content="{description}">
<link rel="stylesheet" href="/style.css">
</head>
<body>
<div class="wrapper">
<nav class="sidebar">
  <div class="logo"><a href="/" style="text-decoration:none;color:inherit">\u03a6\u039c\u039b</a></div>
  <div class="subtitle">The Open Manual Archive</div>
  <ul>
{nav_html(current_slug)}
  </ul>
</nav>
<main>
<h1>{title}</h1>
{content_html}
<div class="doc-footer">
  <a href="/">\u03a6\u039c\u039b</a> &middot; <a href="https://github.com/Beach-Bum/OMA-Library-OS">Source</a>
</div>
</main>
</div>
</body>
</html>"""


def build():
    os.makedirs(OUT_DIR, exist_ok=True)

    for slug, title in PAGES:
        src = os.path.join(CONTENT_DIR, f"{slug}.mdx")
        with open(src, encoding="utf-8") as f:
            raw = f.read()

        meta, md = parse_frontmatter(raw)
        content = md_to_html(md)
        description = meta.get("seo", meta.get("description", ""))

        out_name = "index.html" if slug == "index" else f"{slug}.html"
        out_path = os.path.join(OUT_DIR, out_name)

        with open(out_path, "w", encoding="utf-8") as f:
            f.write(doc_template(title, content, slug, description, out_name))

        print(f"  {out_path}")

    print(f"\nBuilt {len(PAGES)} pages.")


if __name__ == "__main__":
    build()
