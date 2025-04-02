import tokenlist from "../../../data/tokens/list.json";
import bl_bl from "../assets/visuals/tokens/bl_bl.svg";
import bl_cy from "../assets/visuals/tokens/bl_cy.svg";
import bl_gr from "../assets/visuals/tokens/bl_gr.svg";
import bl_re from "../assets/visuals/tokens/bl_re.svg";
import bl_ye from "../assets/visuals/tokens/bl_ye.svg";
import cy_bl from "../assets/visuals/tokens/cy_bl.svg";
import cy_cy from "../assets/visuals/tokens/cy_cy.svg";
import cy_gr from "../assets/visuals/tokens/cy_gr.svg";
import cy_re from "../assets/visuals/tokens/cy_re.svg";
import cy_ye from "../assets/visuals/tokens/cy_ye.svg";
import pu_bl from "../assets/visuals/tokens/pu_bl.svg";
import pu_cy from "../assets/visuals/tokens/pu_cy.svg";
import pu_gr from "../assets/visuals/tokens/pu_gr.svg";
import pu_re from "../assets/visuals/tokens/pu_re.svg";
import pu_ye from "../assets/visuals/tokens/pu_ye.svg";
import re_bl from "../assets/visuals/tokens/re_bl.svg";
import re_cy from "../assets/visuals/tokens/re_cy.svg";
import re_gr from "../assets/visuals/tokens/re_gr.svg";
import re_re from "../assets/visuals/tokens/re_re.svg";
import re_ye from "../assets/visuals/tokens/re_ye.svg";
import ye_bl from "../assets/visuals/tokens/ye_bl.svg";
import ye_cy from "../assets/visuals/tokens/ye_cy.svg";
import ye_gr from "../assets/visuals/tokens/ye_gr.svg";
import ye_re from "../assets/visuals/tokens/ye_re.svg";
import ye_ye from "../assets/visuals/tokens/ye_ye.svg";

const icons = {
  bl_bl: bl_bl,
  bl_cy: bl_cy,
  bl_gr: bl_gr,
  bl_re: bl_re,
  bl_ye: bl_ye,
  cy_bl: cy_bl,
  cy_cy: cy_cy,
  cy_gr: cy_gr,
  cy_re: cy_re,
  cy_ye: cy_ye,
  pu_bl: pu_bl,
  pu_cy: pu_cy,
  pu_gr: pu_gr,
  pu_re: pu_re,
  pu_ye: pu_ye,
  re_bl: re_bl,
  re_cy: re_cy,
  re_gr: re_gr,
  re_re: re_re,
  re_ye: re_ye,
  ye_bl: ye_bl,
  ye_cy: ye_cy,
  ye_gr: ye_gr,
  ye_re: ye_re,
  ye_ye: ye_ye,
};

export default function Token({ tagID }) {
  const token = tokenlist[tagID];

  if (token !== undefined) {
    return (
      <img src={icons[token]} alt="icon" className="option__figure-icon" />
    );
  } else {
    return (
      <svg
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
    );
  }
}
