import logging, json, sys

def _json_formatter(record: logging.LogRecord) -> str:
    payload = {
        "level": record.levelname,
        "name": record.name,
        "msg": record.getMessage(),
    }
    if record.exc_info:
        payload["exc_info"] = True
    return json.dumps(payload)

class JsonFormatter(logging.Formatter):
    def format(self, record):
        return _json_formatter(record)

def configure(json_mode: bool = True, level: int = logging.INFO):
    logger = logging.getLogger()
    logger.setLevel(level)
    logger.handlers.clear()
    handler = logging.StreamHandler(sys.stdout)
    handler.setLevel(level)
    handler.setFormatter(JsonFormatter() if json_mode else logging.Formatter("%(levelname)s %(name)s: %(message)s"))
    logger.addHandler(handler)
