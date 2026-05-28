from __future__ import annotations

from dataclasses import dataclass
import importlib.util
from pathlib import Path

BRANDING_HELPER_PATH = Path("/usr/lib/keskos/branding.py")


@dataclass(frozen=True)
class Branding:
    name: str = "KeskOS"
    pretty_name: str = "KeskOS"
    layer: str = ""
    layer_name: str = ""
    brand_line: str = "KeskOS"
    channel: str = "stable"
    build_id: str = "dev"
    accent_color: str = "#ce6a35"
    home_url: str = "https://keskos.org"
    documentation_url: str = "https://docs.keskos.org"
    download_url: str = "https://downloads.keskos.org"
    support_url: str = "https://docs.keskos.org"
    bug_report_url: str = "https://github.com/KeskOS"

    @property
    def spaced_name(self) -> str:
        return " ".join(list(self.name.upper()))


def load_branding() -> Branding:
    if not BRANDING_HELPER_PATH.is_file():
        return Branding()

    spec = importlib.util.spec_from_file_location("keskos_branding", BRANDING_HELPER_PATH)
    if spec is None or spec.loader is None:
        return Branding()

    module = importlib.util.module_from_spec(spec)
    try:
        spec.loader.exec_module(module)
    except Exception:
        return Branding()

    payload: dict[str, str] = {}
    for attr, key in (
        ("OS_NAME", "name"),
        ("OS_PRETTY_NAME", "pretty_name"),
        ("OS_LAYER", "layer"),
        ("OS_LAYER_NAME", "layer_name"),
        ("OS_BRAND_LINE", "brand_line"),
        ("OS_CHANNEL", "channel"),
        ("OS_BUILD_ID", "build_id"),
        ("OS_ACCENT_COLOR", "accent_color"),
        ("OS_HOME_URL", "home_url"),
        ("OS_DOCUMENTATION_URL", "documentation_url"),
        ("OS_DOWNLOAD_URL", "download_url"),
        ("OS_SUPPORT_URL", "support_url"),
        ("OS_BUG_REPORT_URL", "bug_report_url"),
    ):
        value = getattr(module, attr, None)
        if isinstance(value, str) and value.strip():
            payload[key] = value.strip()

    branding = Branding(**{**Branding().__dict__, **payload})
    if not branding.layer_name and branding.layer:
        return Branding(**{**branding.__dict__, "layer_name": f"Layer {branding.layer}"})
    return branding
