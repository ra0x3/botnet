class _BitsyError(Exception):
    code: int


class RequestError(Exception):
    code = 500


class InvalidPermissionError(_BitsyError):
    code = 500


class InvalidSettingError(_BitsyError):
    code = 500


class WebError(_BitsyError):
    code = 500


class DatabaseError(_BitsyError):
    code = 500


class VaultError(_BitsyError):
    code = 500


class ResourceDoesNotExist(_BitsyError):
    code = 500


class ExpiredAccessTokenError(_BitsyError):
    code = 500
