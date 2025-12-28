<!--
SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>

SPDX-License-Identifier: LGPL-3.0-or-later
-->

NagoyaAPI
===

Simple API providing a lookup on whether measures implementing
the [Nagoya Protocol](https://en.wikipedia.org/wiki/Nagoya_Protocol) are to be respected regarding
a dataset.

The API aims to be integrated into pipelines or portals like a search portal to provide information to researchers.

Applicability of the Measures
---

If a country implements ABS measures based on the Nagoya Protocol, they need to be followed. This
[Checklist](https://www.research-operations.admin.cam.ac.uk/nagoya-checklist-part-1) can be helpful to work out the
details.

Configuration
---

The Service can be configured using a .env file.

| Option         | Type        | Default | Required? | Description                                            |
|----------------|-------------|---------|-----------|--------------------------------------------------------| 
| SERVER_HOST    | IP Address  | 0.0.0.0 | Yes       | IP address to bind the server to                       |
| SERVER_PORT    | Port Number | 3125    | Yes       | Port to bind the server to                             |
| NOMINATIM_HOST | URL         | None    | Yes       | (External) Nominatim Host to use for reverse geocoding |

Usage
---

The service exposes several endpoints to check whether a country has ABS measures which (potentially) need to be
respected according to the Nagoya Protocol. The Endpoints either take a ISO 3166 Country Code or geographic coordinates.

Endpoints
----

| Method | Path                | Description                                                                           |
|--------|---------------------|---------------------------------------------------------------------------------------|
| POST   | `/nagoya_check_cc`  | Perform a Nagoya compliance check using a country code.                               |
| POST   | `/nagoya_check_geo` | Perform a Nagoya compliance check using geographic coordinates (latitude, longitude). |
| GET    | `/health`           | Simple health‑check endpoint returning service status.                                |
| GET    | `/openapi.json`     | Retrieve the OpenAPI specification in JSON format.                                    |
| GET    | `/swagger-ui`       | Interactive Swagger UI for exploring the API.                                         |

Other Solutions
---

[Nagoya Lookup Service](https://github.com/hseifert/nagoya-lookup-service): Provides Lookup based on coordinates and is
currently (2025-10-04) unmaintained. As information regarding Nagoya Measures the service just uses the information
whether a country has ratified the Protocol, but not, whether it actually implements measures. Those can (now) be
accessed via the (upcoming) API of the [ABS Clearing House](https://absch.cbd.int). This is used for the NagoyaAPI
Project.

Additional Resources
---

* Information on the Nagoya Protocol and ongoing discussions: [Website of the CBD](https://www.cbd.int/abs)
* [Information regarding the Convention on Biological Diversity](https://www.cbd.int)
* [Ongoing Discussion on the inclusion of Digital Sequence Information](https://www.cbd.int/dsi-gr) into an Access and
  Benefit Sharing
  Mechanism
* [EU Regulation on the Nagoya Protocol](https://eur-lex.europa.eu/eli/reg/2014/511/oj/eng), mainly concerning
  documentation and due diligence
