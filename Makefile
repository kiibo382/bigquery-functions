cp-output:
	cp -r output ./bigquery-functions && cp -r output ./bigquery-functions-types-macros

# Notes: If you use sysmlink, it will not work properly on windows and so on, so use cp-output above.
make-output-symlink:
	ln -s ./outpt ./bigquery-functions/output && ln -s ./outpt ./bigquery-functions-types-macros/output

publish-types-macros:
	cd bigquery-functions-types-macros && cargo publish

publish:
	cd bigquery-functions && cargo publish
