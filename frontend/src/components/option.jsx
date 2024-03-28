import Shape from "./shape";

export default function Option({ label, tags }) {
    return (
        <div className="option">
            <div className={tags != 0 ? "option__content option__content--showamount" : "option__content"}>
                {tags != 0 && <div className="option__amount">
                    {tags}
                </div>}
                <div className="option__label">
                    {label}
                </div>
                {tags != 0 && <div className="option__silhouettes">
                    {[...Array(tags)]
                        .map((i, e) => (
                            <Shape
                                key={e}
                                shape={"figure"}
                                className="figure"
                            />
                        ))}
                </div>}
            </div>
        </div>
    )
}