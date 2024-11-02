# Contact De-duper

Usage:
`decard duplicates.vcf deduped.vcf`

## Notes:
1. Matches against Full Name (FN) property
2. Does not account for multiple phone numbers per contact
3. Does account for and merge different properties
4. Fixes some corruptions, like missing telephone params
