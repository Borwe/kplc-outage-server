# KPLC OUTAGE SERVER

This is a server that parses the kplc data using the rust library [kplc-outage-parser](https://github.com/Borwe/kplc-outage-parser)

## How to use:
Simply send a `POST` request to the url `https://kplc.fly.dev/` which is where this server is hosted for public use.
In the `POST` request the data must be of `Content-Type: application/json` and contain the following
setup:
```
{"url":"<kplc url to the pdf to be parsed>"}
```

### Example:
```
{"url":"https://www.kplc.co.ke/img/full/Interruptions - 02.02.2023.pdf"}
```

One can use curl to do the requests as shown bellow:

```
curl https://kplc.fly.dev/ --data '{"url":"https://www.kplc.co.ke/img/full/Interruptions - 02.02.2023.pdf"}' -H "Content-Type: application/json"
```


The json spitted out would look like the following bellow


### When success:
```
{
  "success": true,
  "data": {
    "regions": [
      {
        "region": "NAIROBI                                                                ",
        "parts": [
          {
            "part": "",
            "areas": [
              {
                "area": "GRAIN BULK",
                "places": [
                  "Heavy Engineering",
                  "Grain Bulk Handlers",
                  "Posh Auto body",
                  "SGR Head office& adjacent customers."
                ],
                "date": {
                  "day": 5,
                  "month": 2,
                  "year": 2023
                },
                "time": {
                  "start": "9.00AM",
                  "end": "5.00PM"
                }
              },
            ]
          },
          {
            "part": ...
          },
        ]
      },
      {
        "region": ...
      },
    ]
  },
  "message": "success"
}
```

### When failure:


```
{
  "success": false,
  "data": null,
  "message": "Reason for failure"
}
```

