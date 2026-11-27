---
type: Playbook
title: "knowledge / references の RAG 版"
description: "~/knowledge と ~/references を検索可能にするためのコーパス、評価クエリ、再現手順。"
tags:
  - rag
  - knowledge-management
  - okf
generated:
  by: agent:dsh
  at: "2026-11-28T00:50:00+09:00"
verified: []
status: stable
stale_after: "2027-02-28T00:00:00+09:00"
---

# knowledge / references の RAG 版

`~/knowledge` の知見を、質問すると関連する文章を探してくれる形にしたもの。
検索エンジンは `~/document/deepseek-harness/default-workspace/rag_poc/` を使う。

## 対象

| ディレクトリ | 役割 |
| --- | --- |
| `corpus/` | frontmatter を除いた本文（4 ファイル / 56 チャンク） |
| `manifest.json` | OKF メタデータ（type / title / tags / stale_after / verified / sources） |
| `qrels.json` | 評価用クエリ（簡易 3 件 + held-out 6 件） |
| `eval-results.json` | 4 モードの評価結果 |
| `build_corpus.py` | 元文書から `corpus/` と `manifest.json` を再生成する |

`index.md` / `log.md` / `todo.md` / `AGENTS.md` / `inbox/` は検索対象外。
`~/references`（21,036 ファイル）は混ぜず、必要になったら別コレクションで運用する。

## 再現手順

```bash
python3 ~/knowledge/rag/build_corpus.py

cd ~/document/deepseek-harness/default-workspace/rag_poc
export RAG_CORPUS_DIR=~/knowledge/rag/corpus
export RAG_COLLECTION=knowledge_concepts
export RAG_ALLOW_EXTERNAL_CORPUS=1
export EMBED_MODEL=kun432/cl-nagoya-ruri-base
export RAG_VECTOR_SIZE=768
export CHUNK_SIZE=300
export CHUNK_OVERLAP=50
export RAG_EMBED_PREFIX_QUERY='クエリ: '
export RAG_EMBED_PREFIX_PASSAGE='文章: '
python3 ingest.py
python3 query.py 'Pin<&mut Self> が必要な理由は？' --mode hybrid --json
python3 evaluate.py --qrels ~/knowledge/rag/qrels.json \
  --output ~/knowledge/rag/eval-results.json
```

`RAG_ALLOW_EXTERNAL_CORPUS=1` はリポジトリ配下以外のコーパスを使うための明示的な許可。
設定しない場合は従来どおりリポジトリ配下以外を拒否する。

## 評価結果（2026-11-28）

held-out recall@1 / nDCG@10:

| モード | recall@1 | nDCG@10 |
| --- | --- | --- |
| keyword | 1.0 | 1.0 |
| hybrid | 1.0 | 1.0 |
| dense | 0.8333 | 0.9167 |
| hybrid-rerank | 0.5〜0.6667 | 0.753〜0.8145 |

このコーパスでは `keyword` / `hybrid` が最良。`hybrid-rerank` は LLM 呼び出しで
遅く（約 3.7 秒）、順位が悪化するため常用しない。
