#!/usr/bin/env bash
set -euo pipefail
mkdir -p assets/media/images

make_svg() {
  local file="$1" bg1="$2" bg2="$3" accent="$4" accent2="$5" body="$6" body2="$7" string="$8" glow="$9"
  cat > "$file" <<SVG
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" role="img" aria-label="Bass guitar avatar variant">
  <defs>
    <radialGradient id="bg" cx="35%" cy="22%" r="82%">
      <stop offset="0" stop-color="$bg1"/>
      <stop offset="0.58" stop-color="$bg2"/>
      <stop offset="1" stop-color="#05060a"/>
    </radialGradient>
    <linearGradient id="body" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="$body"/>
      <stop offset="1" stop-color="$body2"/>
    </linearGradient>
    <linearGradient id="trim" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="$accent"/>
      <stop offset="1" stop-color="$accent2"/>
    </linearGradient>
    <filter id="shadow" x="-35%" y="-35%" width="170%" height="170%">
      <feDropShadow dx="0" dy="24" stdDeviation="25" flood-color="#000" flood-opacity="0.48"/>
    </filter>
    <filter id="softGlow" x="-40%" y="-40%" width="180%" height="180%">
      <feGaussianBlur stdDeviation="18" result="blur"/>
      <feColorMatrix in="blur" type="matrix" values="0 0 0 0 0.9 0 0 0 0 0.7 0 0 0 0 0.3 0 0 0 0.35 0"/>
      <feMerge><feMergeNode/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
  </defs>

  <rect width="1024" height="1024" rx="224" fill="url(#bg)"/>
  <circle cx="512" cy="512" r="386" fill="none" stroke="$accent" stroke-opacity="0.16" stroke-width="9"/>
  <circle cx="512" cy="512" r="304" fill="none" stroke="$glow" stroke-opacity="0.16" stroke-width="4"/>

  <!-- subtle C / portfolio mark -->
  <path d="M635 262c-142-72-322 4-382 154-67 169 42 344 210 367 96 13 185-21 248-82" fill="none" stroke="$accent" stroke-width="54" stroke-linecap="round" opacity="0.20"/>

  <!-- More rotated bass: the double-lobed body is intentionally B-like. -->
  <g filter="url(#shadow)" transform="rotate(-50 512 512)">
    <!-- neck -->
    <rect x="486" y="104" width="54" height="594" rx="22" fill="#151b27" stroke="$accent" stroke-width="7"/>
    <rect x="503" y="126" width="7" height="538" fill="$string" opacity="0.9"/>
    <rect x="526" y="126" width="7" height="538" fill="$string" opacity="0.9"/>
    <g fill="$accent" opacity="0.78">
      <rect x="480" y="204" width="66" height="5" rx="2"/>
      <rect x="480" y="272" width="66" height="5" rx="2"/>
      <rect x="480" y="340" width="66" height="5" rx="2"/>
      <rect x="480" y="408" width="66" height="5" rx="2"/>
      <rect x="480" y="476" width="66" height="5" rx="2"/>
      <rect x="480" y="544" width="66" height="5" rx="2"/>
      <rect x="480" y="612" width="66" height="5" rx="2"/>
    </g>

    <!-- headstock -->
    <path d="M464 66c0-23 19-42 42-42h58c19 0 32 19 25 37l-27 68c-6 15-20 24-36 24h-20c-23 0-42-19-42-42z" fill="url(#trim)" stroke="$string" stroke-width="6"/>
    <g fill="$string">
      <circle cx="485" cy="68" r="10"/>
      <circle cx="563" cy="70" r="10"/>
      <circle cx="488" cy="120" r="10"/>
      <circle cx="545" cy="120" r="10"/>
    </g>

    <!-- B-like body silhouette: two pronounced lobes on one side, waist in middle -->
    <path d="M459 598
             C405 595 360 628 342 681
             C319 748 360 821 429 837
             C466 846 503 836 532 812
             C567 854 638 862 689 821
             C752 771 746 675 678 631
             C631 601 574 608 535 642
             C516 616 490 601 459 598Z"
          fill="url(#body)" stroke="$string" stroke-width="11"/>
    <path d="M507 641
             C470 632 435 648 419 681
             C400 720 421 764 462 776
             C491 784 520 773 538 751
             C564 787 617 790 650 758
             C688 721 674 657 625 642
             C590 632 558 646 540 674
             C533 658 521 647 507 641Z"
          fill="#070b13" opacity="0.36"/>
    <ellipse cx="521" cy="707" rx="55" ry="39" fill="#060911" opacity="0.82"/>
    <rect x="488" y="625" width="88" height="25" rx="9" fill="$string" opacity="0.95"/>
    <rect x="474" y="767" width="120" height="19" rx="10" fill="$string" opacity="0.92"/>
  </g>

  <path d="M216 846c83 55 184 86 296 86s213-31 296-86" fill="none" stroke="$string" stroke-opacity="0.12" stroke-width="18" stroke-linecap="round"/>
</svg>
SVG
}

make_svg assets/media/images/bass-midnight-gold_avatar.svg '#29313f' '#101723' '#f5d06f' '#9d6421' '#d19031' '#704111' '#fff1bd' '#8b5cf6'
make_svg assets/media/images/bass-oxide-teal_avatar.svg '#12313a' '#08151a' '#5eead4' '#0f766e' '#f97316' '#7c2d12' '#d5fff7' '#ffb86b'
make_svg assets/media/images/bass-plum-mint_avatar.svg '#33213e' '#120a1c' '#a7f3d0' '#34d399' '#c084fc' '#6d28d9' '#f0fdf4' '#22d3ee'
make_svg assets/media/images/bass-cream-ink_avatar.svg '#f1dcc2' '#2a211b' '#f6c35b' '#6f4b16' '#18181b' '#3f2f1f' '#fff7e6' '#f59e0b'

for svg in assets/media/images/*.svg; do
  png="${svg%.svg}.png"
  magick -background none "$svg" -resize 1024x1024 "$png"
done

magick assets/media/images/bass-midnight-gold_avatar.png -resize 512x512 /tmp/avatar-midnight.png
magick assets/media/images/bass-oxide-teal_avatar.png -resize 512x512 /tmp/avatar-oxide.png
magick assets/media/images/bass-plum-mint_avatar.png -resize 512x512 /tmp/avatar-plum.png
magick assets/media/images/bass-cream-ink_avatar.png -resize 512x512 /tmp/avatar-cream.png
magick /tmp/avatar-midnight.png /tmp/avatar-oxide.png +append /tmp/avatar-row1.png
magick /tmp/avatar-plum.png /tmp/avatar-cream.png +append /tmp/avatar-row2.png
magick /tmp/avatar-row1.png /tmp/avatar-row2.png -append assets/media/images/bass-avatars_contact-sheet.png

identify assets/media/images/bass-avatars_contact-sheet.png
