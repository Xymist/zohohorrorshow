# ZohoHorrorShow

![](https://travis-ci.org/Xymist/zohohorrorshow.svg?branch=master)

A library for interacting with the Zoho Projects API, because it's an awful nightmare of ambiguous fields,
optional fields, random casing and largely absent documentation.

Work in progress:

- Finish defining structs for all postable and retrieveable objects
- Finish implementing all GET, PUT, POST and DELETE urls
- Consistently wrap inconsistent fields (no numeric strings!)
- Make sure everything that might be nil is wrapped in an Option (This isn't documented so will probably be a matter of experimentation)
- Better documentation
- Deduplicate with macros where possible
