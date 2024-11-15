# Redactor

A Rust-based text redaction service utilizing Named Entity Recognition (NER) to automatically redact sensitive information from text.

> ⚠️ **Note**: The redaction functionality is currently under development.

## Overview

Redactor is built with:
- Rust
- Nix
- Docker
- Hugging Face Hub integration

## Features

### REST API Endpoints

#### GET|PUT `/api/options`
Configure the NER model and entity types for redaction.

✅ Features:
- Automatic model downloading via Hugging Face Hub
- Model file caching
- Request validation
- Bad Request (400) responses for unsupported entity types

Example request:
```json
{
    "type": "ner",
    "model_name": "dslim/bert-base-NER",
    "entity_types": ["PER", "ORG", "LOC"]
}
