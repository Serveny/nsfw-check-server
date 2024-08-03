# NSFW Check Server

A web server that checks if an image is NSFW.

## API

#### Public requests

<table>
  <tr>
    <th>Function</th>
    <th>Method</th>
    <th>URL</th>
    <th>Parameters</th>
    <th>Return format</th>
    <th>Returns</th>
  </tr>
  <tr>
    <td>Check image from URL</td>
    <td>GET</td>
    <td><code>/check</code></td>
    <td><code>url</code>: URL as string to read image from</td>
    <td>JSON</td>
    <td>JSON encoded result</td>
  </tr>
  <tr>
    <td>Upload and check image</td>
    <td>POST</td>
    <td><code>/check</code></td>
    <td><code>image</code>: Image as form file</td>
    <td>JSON</td>
    <td>JSON encoded result</td>
  </tr>
</table>

## Docker

1. **Build**: `docker build -t nsfw-check-server .`
2. **Run**: `docker run -p 6969:6969 --rm --name ncs nsfw-check-server`
