#!/usr/bin/env python3
"""~/knowledge の OKF 文書を RAG 用コーパスへ整形する。

元の文書は変更しない。frontmatter を rag/manifest.json へ分離し、
rag/corpus/ 配下へ本文だけを相対パス構造のまま書き出す。

使い方:
    python3 ~/knowledge/rag/build_corpus.py
"""

from __future__ import annotations

import json
from pathlib import Path

import yaml

KNOWLEDGE_ROOT = Path(__file__).resolve().parent.parent
RAG_ROOT = KNOWLEDGE_ROOT / "rag"
CORPUS_ROOT = RAG_ROOT / "corpus"

# 正本と写しを混ぜないため、references/ は対象外。
# index/log/todo/AGENTS/inbox はナビゲーション・運用ルールなので除外。
SOURCES = (
    "concepts/languages/rust-nomicon.md",
    "concepts/languages/rust-async-book.md",
    "concepts/languages/rust-by-example.md",
    "skills/rust-advanced.md",
)


class LiteralDateLoader(yaml.SafeLoader):
    """RFC3339 の日付を datetime に変換せず文字列のまま保持する。"""


LiteralDateLoader.yaml_implicit_resolvers = {
    key: [
        (tag, regexp)
        for tag, regexp in resolvers
        if tag != "tag:yaml.org,2002:timestamp"
    ]
    for key, resolvers in yaml.SafeLoader.yaml_implicit_resolvers.items()
}


LOCAL_USER = Path.home().name


def sanitize(text: str) -> str:
    """公開リポジトリ向けにローカル識別子をプレースホルダへ置換する。"""
    return (
        text.replace(f"/home/{LOCAL_USER}", "<home>")
        .replace(f"human:{LOCAL_USER}", "human:<user>")
    )


def split_frontmatter(path: Path) -> tuple[dict, str]:
    """OKF frontmatter と本文を分離する。"""
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return {}, text
    end = text.find("\n---\n", 4)
    if end < 0:
        raise ValueError(f"{path}: frontmatter の終端が無い")
    meta = yaml.load(text[4:end], Loader=LiteralDateLoader) or {}
    body = text[end + 5 :].lstrip("\n")
    return meta, body


def main() -> int:
    CORPUS_ROOT.mkdir(parents=True, exist_ok=True)
    manifest: list[dict] = []
    for rel in SOURCES:
        src = KNOWLEDGE_ROOT / rel
        meta, body = split_frontmatter(src)
        dest = CORPUS_ROOT / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_text(sanitize(body), encoding="utf-8")
        manifest.append(
            {
                "source": rel,
                "type": meta.get("type"),
                "title": meta.get("title"),
                "description": meta.get("description"),
                "tags": meta.get("tags") or [],
                "status": meta.get("status"),
                "stale_after": meta.get("stale_after"),
                "verified": meta.get("verified") or [],
                "sources": meta.get("sources") or [],
            }
        )
    (RAG_ROOT / "manifest.json").write_text(
        sanitize(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n"),
        encoding="utf-8",
    )
    print(f"wrote {len(manifest)} files to {CORPUS_ROOT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
