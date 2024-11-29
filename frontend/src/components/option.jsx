import clsx from "clsx";
import tokenlist from "../data/tokens/list.json";

export default function Option({
  className = "",
  label,
  amount = 0,
  big = false,
  tagIds = [],
}) {
  return (
    <div className={clsx("option", className, { "option--big": big })}>
      {!big && <div className="option__amount">{amount}</div>}
      <div className="option__content">
        <div className="option__label">{label}</div>
        {!big && (
          <div className="option__figure-container">
            {tagIds.map((value, index) =>
              Object.prototype.hasOwnProperty.call(tokenlist, value) ? (
                <div key={index}>icon placeholder</div>
              ) : (
                <svg
                  key={index}
                  className="option__figure-icon"
                  width="15"
                  height="66"
                  viewBox="0 0 15 66"
                  fill="currentColor"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path d="M13.2268 6.81944C13.2268 10.0801 10.583 12.7238 7.32237 12.7238C4.0613 12.7238 1.41797 10.0801 1.41797 6.81944C1.41797 3.55837 4.06139 0.915039 7.32237 0.915039C10.5831 0.915039 13.2268 3.55846 13.2268 6.81944Z" />
                  <path d="M10.5676 14.2434H4.08794C1.98237 14.2434 0.267578 15.9476 0.267578 18.0637V37.0036C0.267578 38.7944 1.50503 40.2922 3.16537 40.7046L4.29427 65.1035L10.3612 65.1039L11.2078 40.77C13.0096 40.4662 14.388 38.8923 14.388 37.0037V18.0529C14.3774 15.9474 12.6732 14.2432 10.5676 14.2432L10.5676 14.2434Z" />
                </svg>
              ),
            )}
          </div>
        )}
      </div>
    </div>
  );
}
