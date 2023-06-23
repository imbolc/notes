Fetch API
=========

Error handling
--------------
```javascript
try {
    let r = await fetch("/foo")
    if (!r.ok) { throw r }
    let data = await r.json()
} catch (e) {
    alert(e.statusText || e.message || e.status)
}
```

Post json
---------
```javascript
try {
    let r = await fetch("/foo", {
        method: "POST",
        body: JSON.stringify(payload),
        credentials: "same-origin",
        headers: {
            "Content-Type": "application/json"
        }
    })
    if (!r.ok) { throw r }
    let data = await r.json()
} catch (e) {
    alert(e.statusText || e.message || e.status)
}
```

Parallel requests
-----------------
```javascript
const urls = [
    "https://httpbin.org/status/400",
    "https://httpbin.org/get",
];
try {
    const [first, second] = await Promise.all(
        urls.map(async (url) => {
            const res = await fetch(url);
            if (!res.ok) {
                throw res;
            }
            console.error(`GET ${url} ${res.status}`);
            return res.json();
        })
    );
} catch (e) {
    alert(e.statusText || e.message || e.status);
}
```
