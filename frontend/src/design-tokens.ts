// Lièvre Design System — Design Tokens
// Import this into Penpot, Tailwind, or CSS custom properties

export const colors = {
  // Brand
  primary: '#E63946',      // Hare Red — CTAs, likes, active states
  dark: '#1D3557',         // Hare Dark — headers, text, nav
  light: '#F1FAEE',        // Hare Light — backgrounds, cards
  gray: '#A8DADC',         // Hare Gray — borders, secondary

  // Activity types
  ride: '#457B9D',         // Blue — cycling
  run: '#2A9D8F',          // Green — running
  swim: '#264653',         // Dark teal — swimming
  walk: '#E9C46A',         // Orange — walking
  hike: '#F4A261',         // Brown — hiking

  // UI
  success: '#2A9D8F',
  warning: '#E9C46A',
  error: '#E63946',
  info: '#457B9D',

  // Neutrals
  white: '#FFFFFF',
  black: '#1A1A1A',
  gray50: '#F9FAFB',
  gray100: '#F3F4F6',
  gray200: '#E5E7EB',
  gray300: '#D1D5DB',
  gray400: '#9CA3AF',
  gray500: '#6B7280',
  gray600: '#4B5563',
  gray700: '#374151',
  gray800: '#1F2937',
  gray900: '#111827',
};

export const typography = {
  fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif',
  sizes: {
    xs: '12px',
    sm: '14px',
    base: '16px',
    lg: '18px',
    xl: '24px',
    '2xl': '32px',
  },
  weights: {
    normal: 400,
    medium: 500,
    semibold: 600,
    bold: 700,
  },
  lineHeights: {
    tight: 1.2,
    normal: 1.5,
    relaxed: 1.6,
  },
};

export const spacing = {
  xs: '4px',
  sm: '8px',
  md: '16px',
  lg: '24px',
  xl: '32px',
  '2xl': '48px',
};

export const borderRadius = {
  sm: '4px',
  md: '6px',
  lg: '8px',
  xl: '12px',
  full: '9999px',
};

export const shadows = {
  sm: '0 1px 2px rgba(0,0,0,0.05)',
  md: '0 2px 4px rgba(0,0,0,0.05)',
  lg: '0 4px 8px rgba(0,0,0,0.1)',
};

// CSS custom properties string — paste into Penpot or index.css
export const cssVariables = `
:root {
  /* Colors */
  --color-primary: ${colors.primary};
  --color-dark: ${colors.dark};
  --color-light: ${colors.light};
  --color-gray: ${colors.gray};
  
  /* Activity */
  --color-ride: ${colors.ride};
  --color-run: ${colors.run};
  --color-swim: ${colors.swim};
  --color-walk: ${colors.walk};
  --color-hike: ${colors.hike};
  
  /* Typography */
  --font-family: ${typography.fontFamily};
  --text-xs: ${typography.sizes.xs};
  --text-sm: ${typography.sizes.sm};
  --text-base: ${typography.sizes.base};
  --text-lg: ${typography.sizes.lg};
  --text-xl: ${typography.sizes.xl};
  --text-2xl: ${typography.sizes['2xl']};
  
  /* Spacing */
  --spacing-xs: ${spacing.xs};
  --spacing-sm: ${spacing.sm};
  --spacing-md: ${spacing.md};
  --spacing-lg: ${spacing.lg};
  --spacing-xl: ${spacing.xl};
  
  /* Borders */
  --radius-sm: ${borderRadius.sm};
  --radius-md: ${borderRadius.md};
  --radius-lg: ${borderRadius.lg};
  
  /* Shadows */
  --shadow-sm: ${shadows.sm};
  --shadow-md: ${shadows.md};
}
`;
