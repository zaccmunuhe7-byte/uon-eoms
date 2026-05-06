export const metadata = {
  title: 'UON-EOMS',
  description: 'UON Electoral & Organization Management System',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
