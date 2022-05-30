module.exports = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      backgroundColor: {
        skin: {
          base: 'var(--color-bg-base)',
          bar: 'var(--color-bg-bar)',
          input: 'var(--color-bg-input)',
          chatArea: 'var(--color-bg-chatArea)',
          bubble: 'var(--color-bg-bubble)'
        }
      },
      textColor: {
        skin: {
          base: 'var(--color-text-base)',
          invert: 'var(--color-text-invert)'
        }
      },
      borderColor: {
        skin: {
          base: 'var(--color-border-base)',
          invert: 'var(--color-border-invert)'
        }
      }
    }
  },
  plugins: [
    ({ addVariant }) => {
      addVariant('children', '& > *')
    }
  ]
}
