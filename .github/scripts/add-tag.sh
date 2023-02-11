# Add tag to GitHub
curl -s -X POST "https://api.github.com/repos/${REPO}/git/refs" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -d @- << EOS
{
  "ref": "refs/tags/${TAG}",
  "sha": "${COMMIT}"
}
EOS
