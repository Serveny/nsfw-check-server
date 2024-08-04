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
    <td>Classification result</td>
  </tr>
  <tr>
    <td>Upload and check image</td>
    <td>POST</td>
    <td><code>/check</code></td>
    <td><code>image</code>: Image as form file</td>
    <td>JSON</td>
    <td>Classification result</td>
  </tr>
  <tr>
    <td>Is image allowed (=not NSFW)</td>
    <td>GET</td>
    <td><code>/is_allowed</code></td>
    <td><code>url</code>: URL as string to read image from</td>
    <td>JSON</td>
    <td>boolean</td>
  </tr>
  <tr>
    <td>Is image allowed (=not NSFW)</td>
    <td>POST</td>
    <td><code>/is_allowed</code></td>
    <td><code>image</code>: Image as form file</td>
    <td>JSON</td>
    <td>boolean</td>
  </tr>
</table>

#### Types

- Classifications result example:

```JSON
[
  { "metric": "Drawings", "score": 0.00016305158 },
  { "metric": "Hentai", "score": 4.0540633e-7 },
  { "metric": "Neutral", "score": 0.9997923 },
  { "metric": "Porn", "score": 0.0000022404822 },
  { "metric": "Sexy", "score": 0.000042102398 }
]
```

## Docker

1. **Build**: `docker build -t nsfw-check-server .`
2. **Run**: `docker run -p 6969:6969 --rm --name ncs nsfw-check-server`
