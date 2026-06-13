import type {CSSProperties, ReactNode, SVGProps} from 'react';
import {useId} from 'react';

import styles from './styles.module.css';

type Props = {
  className?: string;
  title?: string;
  decorative?: boolean;
  size?: number;
};

export default function TikeoLogoMark({
  className,
  title = 'Tikeo orbital control-plane logo',
  decorative = false,
  size,
}: Props): ReactNode {
  const reactId = useId().replace(/:/g, '');
  const coreId = `tikeo-logo-core-${reactId}`;
  const orbitId = `tikeo-logo-orbit-${reactId}`;
  const glowId = `tikeo-logo-glow-${reactId}`;
  const style = size ? ({'--tikeo-logo-size': `${size}px`} as CSSProperties) : undefined;
  const ariaProps: SVGProps<SVGSVGElement> = decorative ? {'aria-hidden': true} : {role: 'img', 'aria-label': title};

  return (
    <svg
      className={[styles.logo, className].filter(Boolean).join(' ')}
      style={style}
      viewBox="0 0 160 160"
      xmlns="http://www.w3.org/2000/svg"
      {...ariaProps}
    >
      {!decorative && <title>{title}</title>}
      <defs>
        <radialGradient id={coreId} cx="48%" cy="42%" r="62%">
          <stop offset="0%" stopColor="var(--tikeo-logo-core-hot)" />
          <stop offset="42%" stopColor="var(--tikeo-logo-primary)" />
          <stop offset="100%" stopColor="var(--tikeo-logo-core-edge)" />
        </radialGradient>
        <linearGradient id={orbitId} x1="24" y1="42" x2="138" y2="118" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="var(--tikeo-logo-primary)" />
          <stop offset="56%" stopColor="var(--tikeo-logo-secondary)" />
          <stop offset="100%" stopColor="var(--tikeo-logo-tertiary)" />
        </linearGradient>
        <filter id={glowId} x="-35%" y="-35%" width="170%" height="170%">
          <feGaussianBlur stdDeviation="3.4" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>
      <circle className={styles.ring} cx="80" cy="80" r="42" />
      <ellipse className={styles.orbit} cx="80" cy="80" rx="58" ry="31" transform="rotate(-22 80 80)" stroke={`url(#${orbitId})`} />
      <ellipse className={styles.orbitMuted} cx="80" cy="80" rx="48" ry="25" transform="rotate(34 80 80)" stroke={`url(#${orbitId})`} />
      <circle className={styles.core} cx="80" cy="80" r="17" fill={`url(#${coreId})`} filter={`url(#${glowId})`} />
      <g className={styles.nodeTrackA}>
        <g transform="rotate(-22 80 80) translate(138 80)">
          <circle className={styles.node} r="7" filter={`url(#${glowId})`} />
          <path className={styles.axis} d="M-4 0h8M0-4v8" />
        </g>
      </g>
      <g className={styles.nodeTrackB}>
        <g transform="rotate(34 80 80) translate(128 80)">
          <circle className={styles.nodeSecondary} r="5.6" filter={`url(#${glowId})`} />
          <path className={styles.axis} d="M-3.2 0h6.4" />
        </g>
      </g>
    </svg>
  );
}
