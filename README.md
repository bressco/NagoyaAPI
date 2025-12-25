<!--
SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>

SPDX-License-Identifier: LGPL-3.0-or-later
-->

NagoyaAPI
===

Simple API providing a lookup on whether measures implementing
the [Nagoya Protocol](https://en.wikipedia.org/wiki/Nagoya_Protocol) are to be respected regarding
a dataset.

The API aims to be integrated into pipelines or portals lie a search portal to provide information to researchers. If
the country affiliation of the researcher is unclear, the pipeline or portal should provide a default one.

Applicability of the Measures
---

Whether measures implementing of the Nagoya Protocol need to be respected follows the Checklist of
the [University of Cambridge](https://www.research-operations.admin.cam.ac.uk/nagoya-checklist-part-1)

Other Solutions
---

[Nagoya Lookup Service](https://github.com/hseifert/nagoya-lookup-service): Provides Lookup based on coordinates and is
currently (2025-10-04) unmaintained. As information regarding Nagoya Measures the service just uses the information
whether a country has ratified the Protocol, but not, whether it actually implements measures. Those can (now) be
accessed via the (upcoming) API of the [ABS Clearing House](absch.cbd.int). This is used for the NagoyaAPI Project.