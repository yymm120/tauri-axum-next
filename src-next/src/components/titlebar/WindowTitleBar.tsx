import styles from "./titlebar.module.css";

interface WindowTitleBarProps {
  //   onClick: () => void
  //   title: string
  //   description: string
}

export const WindowTitleBar: React.FC<WindowTitleBarProps> = ({}: //   onClick,
//   title,
//   description,
WindowTitleBarProps) => (
  <div data-tauri-drag-region className={styles.titlebar}>
    <div className={styles.titlebarbutton} id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div className={styles.titlebarbutton} id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div className={styles.titlebarbutton} id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
  </div>
);
