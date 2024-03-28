type CatalogPanelProps = {}

export default function CatalogPanel(props: CatalogPanelProps) {
    return (
        <iframe
            src="rustdoc/lib_catalog/index.html"
            style={{
                width: "100%",
                height: "calc(100% - 8px)",
                border: "none"
            }}
        />
    );
}
