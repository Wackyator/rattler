from __future__ import annotations

from rattler.version import Version

from rattler.rattler import PyVersionWithSource


class VersionWithSource(Version):
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

    def __init__(self, source: str):
        if isinstance(source, str):
            self._source = source
            super().__init__(source)
            self._inner = PyVersionWithSource(Version(source)._version, source)

        else:
            raise TypeError(
                "VersionWithSource constructor received unsupported type "
                f" {type(source).__name__!r} for the `source` parameter"
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
        >>> v2 = VersionWithSource(str(v.version))
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

    def __str__(self):
        """
        Returns the string representation of the version

        Examples
        --------
        >>> str(VersionWithSource("1.2.3"))
        '1.2.3'
        """
        return self._inner.as_str()

    def __repr__(self):
        """
        Returns a representation of the version

        Examples
        --------
        >>> VersionWithSource("1.2.3")
        VersionWithSource("1.2.3")
        """
        return f'{type(self).__name__}("{self._inner.as_str()}")'

    def __hash__(self) -> int:
        """
        Computes the hash of this instance.

        Examples
        --------
        >>> hash(VersionWithSource("1.2.3")) == hash(VersionWithSource("1.2.3"))
        True
        >>> hash(VersionWithSource("1.2.3")) == hash(VersionWithSource("3.2.1"))
        False
        >>> hash(VersionWithSource("1")) == hash(VersionWithSource("1.0.0"))
        False
        """
        return self._inner.__hash__()

    def __eq__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance represents the same version as `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") == VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("3.2.1") == VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("1") == VersionWithSource("1.0.0")
        False
        """
        return self._inner == other._inner

    def __ne__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance represents the same version as `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") != VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("3.2.1") != VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1") != VersionWithSource("1.0.0")
        True
        """
        return self._inner != other._inner

    def __gt__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance should be ordered *after* `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") > VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("1.2.4") > VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1.2.3.1") > VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("3.2.1") > VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1") > VersionWithSource("1.0.0")
        False
        """
        return self._inner > other._inner

    def __lt__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance should be ordered *before* `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") < VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("1.2.3") < VersionWithSource("1.2.4")
        True
        >>> VersionWithSource("1.2.3") < VersionWithSource("1.2.3.1")
        True
        >>> VersionWithSource("3.2.1") < VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("1") < VersionWithSource("1.0.0")
        True
        """
        return self._inner < other._inner

    def __ge__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance should be ordered *after* or at the same location
        as `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") >= VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1.2.4") >= VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1.2.3.1") >= VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("3.2.1") >= VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1.2.3") >= VersionWithSource("3.2.1")
        False
        >>> VersionWithSource("1") >= VersionWithSource("1.0.0")
        False
        """
        return self._inner >= other._inner

    def __le__(self, other: VersionWithSource) -> bool:
        """
        Returns True if this instance should be ordered *before* or at the same
        location as `other`.

        Examples
        --------
        >>> VersionWithSource("1.2.3") <= VersionWithSource("1.2.3")
        True
        >>> VersionWithSource("1.2.3") <= VersionWithSource("1.2.4")
        True
        >>> VersionWithSource("1.2.3") <= VersionWithSource("1.2.3.1")
        True
        >>> VersionWithSource("3.2.1") <= VersionWithSource("1.2.3")
        False
        >>> VersionWithSource("1") <= VersionWithSource("1.0.0")
        True
        """
        return self._inner <= other._inner
