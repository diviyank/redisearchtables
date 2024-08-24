from importlib.metadata import version

from ._internal import (
    )

__version__ = version("redisearchtables")

__all__ = ["__version__", "RediSearchTable", "RedisPool"]
