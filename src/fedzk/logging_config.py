import json
import logging
import sys


def _json_formatter(record: logging.LogRecord) -> str:
    payload: dict[str, str | bool] = {
        "level": record.levelname,
        "name": record.name,
        "msg": record.getMessage(),
    }
    if record.exc_info:
        payload["exc_info"] = True
    return json.dumps(payload)


class JsonFormatter(logging.Formatter):
    def format(self, record: logging.LogRecord) -> str:
        return _json_formatter(record)


def configure(json_mode: bool = True, level: int = logging.INFO) -> None:
    logger = logging.getLogger()
    logger.setLevel(level)
    logger.handlers.clear()
    handler = logging.StreamHandler(sys.stdout)
    handler.setLevel(level)
    handler.setFormatter(
        JsonFormatter()
        if json_mode
        else logging.Formatter("%(levelname)s %(name)s: %(message)s")
    )
    logger.addHandler(handler)


# NOTE: Call `configure(json_mode=True)` at CLI startup for JSON logs in experiments.
