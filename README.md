# NSFW Check Server

A web server that checks if an image is NSFW.

It's just the NSFW crate (https://github.com/fyko/nsfw) with an actix web server.

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
    <td><code>url</code> as query parameter to read image from</td>
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
    <td><code>url</code> as query parameter to read image from</td>
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

#### Example requests

See under `examples/requests.html` for an interactive example.

- GET: `http://localhost:6969/check?url=https://upload.wikimedia.org/wikipedia/commons/thumb/4/4d/Cat_November_2010-1a.jpg`
- POST: `http://localhost:6969/check` with form data

- GET: `http://localhost:6969/is_allowed?url=https://upload.wikimedia.org/wikipedia/commons/thumb/4/4d/Cat_November_2010-1a.jpg`
- POST: `http://localhost:6969/is_allowed` with form data

## Docker Hub (easiest)

1. **Pull**: `docker pull serveny/nsfw-check-server` (Build available for arm & amd)
2. **Run**: `docker run -p 6969:6969 --rm --name ncs serveny/nsfw-check-server`

## Docker

1. Download this repository and open terminal inside the repository directory
2. **Build**: `docker build -t nsfw-check-server .`
3. **Run**: `docker run -p 6969:6969 --rm --name ncs nsfw-check-server`
