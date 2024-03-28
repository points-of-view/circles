export default function Option({ label }) {
    return (
        <div className="option">
            <div className="option__container">
                <div className="option__content">
                    {label}
                </div>
            </div>
        </div>
    )
}