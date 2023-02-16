cp-output:
	cp -r output ./bigquery-functions && cp -r output ./bigquery-functions-types-macros

publish-types-macros:
	cd bigquery-functions-types-macros && cargo publish

publish:
	cd bigquery-functions && cargo publish
