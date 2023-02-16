cp-output:
	cp -r output ./bigquery-functions && cp -r output ./bigquery-functions-types-macros

ln-output:
	ln -s ./output ./bigquery-functions/output && ln -s ./output ./bigquery-functions-types-macros/output

publish-types-macros:
	cd bigquery-functions-types-macros && cargo publish

publish:
	cd bigquery-functions && cargo publish
