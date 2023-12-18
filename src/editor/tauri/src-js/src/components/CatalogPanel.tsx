type CatalogPanelProps = {}

export default function CatalogPanel(props: CatalogPanelProps) {
    return (
        <iframe
            src="http://localhost:5555"
            style={{
                width: "100%",
                height: "calc(100% - 8px)",
                border: "none"
            }}
        />
    );
}
