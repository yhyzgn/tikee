interface TikeeLogoProps {
  size?: number;
  className?: string;
  showWordmark?: boolean;
}

export function TikeeLogo({ size = 44, className = '', showWordmark = false }: TikeeLogoProps) {
  const classes = ['tikee-logo', showWordmark ? 'tikee-logo--with-wordmark' : '', className].filter(Boolean).join(' ');
  return (
    <span className={classes} aria-label="tikee task orchestration logo" role="img">
      <svg className="tikee-logo__mark" width={size} height={size} viewBox="0 0 64 64" fill="none" aria-hidden="true">
        <defs>
          <linearGradient id="tikee-logo-shell" x1="8" y1="7" x2="57" y2="58" gradientUnits="userSpaceOnUse">
            <stop stopColor="var(--app-primary-color)" />
            <stop offset="0.54" stopColor="var(--app-info-color)" />
            <stop offset="1" stopColor="#7c3aed" />
          </linearGradient>
          <linearGradient id="tikee-logo-flow" x1="14" y1="42" x2="51" y2="20" gradientUnits="userSpaceOnUse">
            <stop stopColor="#e0f2fe" />
            <stop offset="0.45" stopColor="#ffffff" />
            <stop offset="1" stopColor="#dbeafe" />
          </linearGradient>
          <filter id="tikee-logo-glow" x="-30%" y="-30%" width="160%" height="160%" colorInterpolationFilters="sRGB">
            <feDropShadow dx="0" dy="8" stdDeviation="6" floodColor="var(--app-primary-color)" floodOpacity="0.28" />
          </filter>
        </defs>
        <rect className="tikee-logo__shell" x="6" y="6" width="52" height="52" rx="18" fill="url(#tikee-logo-shell)" filter="url(#tikee-logo-glow)" />
        <path className="tikee-logo__orbit" d="M17 41 C25 26 34 49 47 22" stroke="rgba(255,255,255,0.34)" strokeWidth="12" strokeLinecap="round" />
        <path className="tikee-logo__track" d="M16 42 C24 27 34 49 48 21" stroke="url(#tikee-logo-flow)" strokeWidth="5.6" strokeLinecap="round" />
        <path className="tikee-logo__flow" d="M16 42 C24 27 34 49 48 21" stroke="#ffffff" strokeWidth="5.6" strokeLinecap="round" pathLength="100" />
        <path className="tikee-logo__tick" d="M38 24 L47.6 20.6 L45.8 30.6" stroke="#ffffff" strokeWidth="4.8" strokeLinecap="round" strokeLinejoin="round" />
        <circle className="tikee-logo__node tikee-logo__node--one" cx="16" cy="42" r="5.2" fill="#ffffff" />
        <circle className="tikee-logo__node tikee-logo__node--two" cx="31.5" cy="34" r="5.2" fill="#ffffff" />
        <circle className="tikee-logo__node tikee-logo__node--three" cx="48" cy="21" r="5.2" fill="#ffffff" />
        <circle className="tikee-logo__spark" cx="47.5" cy="20.5" r="2" fill="var(--app-info-color)" />
      </svg>
      {showWordmark ? <span className="tikee-logo__wordmark">tikee</span> : null}
    </span>
  );
}
