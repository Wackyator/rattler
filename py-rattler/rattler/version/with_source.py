from __future__ import annotations
from typing import Optional

from rattler.version import Version, VersionBase
from rattler.rattler import PyVersionWithSource


class VersionWithSource(VersionBase):
    """
    Holds a version and the string it was created from. This is useful if
    you want to retain the original string the version was created from.
    This might be useful in cases where you have multiple strings that
    are represented by the same [`Version`] but you still want to be able to
    distinguish them.

    The string `1.0` and `1.01` represent the same version. When you print
    the parsed version though it will come out as `1.0`. You loose the
    original representation. This struct stores the original source string.
    """

    def __init__(self, source: str, version: Optional[Version] = None):
        if isinstance(source, str):
            if version is None:
                version = Version(source)

            if isinstance(version, Version):
                # maybe we should use _inner for inner FFI objects everywhere?
                self._inner = PyVersionWithSource(version._version, source)
                super().__init__(
                    self._inner, VersionWithSource._from_py_version_with_source
                )
            else:
                raise TypeError(
                    "VersionWithSource constructor received unsupported type "
                    f" {type(version).__name__!r} for the `version` parameter"
                )
        else:
            raise TypeError(
                "VersionWithSource constructor received unsupported type "
                f" {type(version).__name__!r} for the `source` parameter"
            )

    @property
    def version(self) -> Version:
        """
        Returns the `Version` from current object.

        Examples
        --------
        >>> v = VersionWithSource("1.0.0")
        >>> v.version
        Version("1.0.0")
        >>> v2 = VersionWithSource("1.0.0", v.version)
        >>> v2.version
        Version("1.0.0")
        """
        return Version._from_py_version(self._inner.version())

    @classmethod
    def _from_py_version_with_source(
        cls, py_version_with_source: PyVersionWithSource
    ) -> VersionWithSource:
        """Construct Rattler VersionWithSource from FFI PyVersionWithSource object."""
        version = cls.__new__(cls)
        version._inner = py_version_with_source
        return version
