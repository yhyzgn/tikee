import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import type {ReactNode} from 'react';

import TikeoLogoMark from '@site/src/components/TikeoLogoMark';

export default function NavbarLogo(): ReactNode {
  const {
    siteConfig: {title},
  } = useDocusaurusContext();
  const homeUrl = useBaseUrl('/');

  return (
    <Link className="navbar__brand" to={homeUrl} aria-label={`${title} home`}>
      <TikeoLogoMark className="navbar__logo" decorative size={32} />
      <b className="navbar__title text--truncate">{title}</b>
    </Link>
  );
}
