# MaterialSpace Data Schema Specification

This document defines the JSON data schema for materials, individual properties, and sources in MaterialSpace.

---

## 1. Material Record Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "MaterialRecord",
  "type": "object",
  "required": ["id", "name", "category", "properties"],
  "properties": {
    "id": {
      "type": "string",
      "description": "Unique URL-safe alphanumeric slug identifier."
    },
    "name": {
      "type": "string",
      "description": "Human-readable full material name."
    },
    "category": {
      "type": "string",
      "enum": ["metal", "ceramic", "polymer", "composite", "glass", "pure_substance", "other"],
      "description": "Primary material classification."
    },
    "composition": {
      "type": ["string", "null"],
      "description": "Chemical formula or nominal elemental alloy composition."
    },
    "grade": {
      "type": ["string", "null"],
      "description": "Standard engineering grade, temper, or heat treatment specification."
    },
    "description": {
      "type": ["string", "null"],
      "description": "Concise engineering summary and typical applications."
    },
    "properties": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/definitions/MaterialProperty"
      }
    },
    "source_ids": {
      "type": "array",
      "items": { "type": "string" },
      "description": "List of reference citation source IDs."
    }
  },
  "definitions": {
    "MaterialProperty": {
      "type": "object",
      "required": ["value", "unit", "source_id"],
      "properties": {
        "value": {
          "type": "number",
          "description": "Normalized numerical magnitude in standard SI base unit."
        },
        "unit": {
          "type": "string",
          "description": "SI unit string (e.g. 'kg/m^3', 'W/(m*K)', 'Pa', '1/K', 'J/(kg*K)', 'K')."
        },
        "source_id": {
          "type": "string",
          "description": "Citation source key referencing sources.json."
        },
        "condition": {
          "type": ["string", "null"],
          "description": "Reference state or testing conditions (e.g. 'Room Temp', '0.2% offset', '0° longitudinal')."
        },
        "original_value": {
          "type": ["number", "string", "null"],
          "description": "Original raw value prior to SI unit normalization."
        },
        "original_unit": {
          "type": ["string", "null"],
          "description": "Original unit prior to SI unit normalization."
        },
        "notes": {
          "type": ["string", "null"]
        }
      }
    }
  }
}
```
