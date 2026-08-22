import { test, expect } from '@playwright/test'

test.describe('HEIC Converter E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // 导航到应用页面
    // 注意：这需要在 tauri dev 或构建后运行
    await page.goto('http://localhost:1420')
  })

  test('should load the application', async ({ page }) => {
    await expect(page.title()).resolves.toContain('HEIC')
  })

  test('should display main view', async ({ page }) => {
    // 检查主视图元素
    const mainView = page.locator('[data-testid="main-view"]')
    await expect(mainView).toBeVisible()
  })

  test('should display title bar', async ({ page }) => {
    const titleBar = page.locator('[data-testid="title-bar"]')
    await expect(titleBar).toBeVisible()
  })

  test('should display file list area', async ({ page }) => {
    const fileListArea = page.locator('[data-testid="file-list-area"]')
    await expect(fileListArea).toBeVisible()
  })

  test('should display settings panel', async ({ page }) => {
    const settingsPanel = page.locator('[data-testid="settings-panel"]')
    await expect(settingsPanel).toBeVisible()
  })

  test('should display status bar', async ({ page }) => {
    const statusBar = page.locator('[data-testid="status-bar"]')
    await expect(statusBar).toBeVisible()
  })

  test('should toggle dark mode', async ({ page }) => {
    const themeButton = page.locator('[data-testid="theme-toggle"]')
    await themeButton.click()
    
    // 检查主题切换
    const html = page.locator('html')
    await expect(html).toHaveClass(/dark|light/)
  })

  test('should display format selector', async ({ page }) => {
    const formatSelector = page.locator('[data-testid="format-selector"]')
    await expect(formatSelector).toBeVisible()
  })

  test('should display quality slider', async ({ page }) => {
    const qualitySlider = page.locator('[data-testid="quality-slider"]')
    await expect(qualitySlider).toBeVisible()
  })

  test('should display add files button', async ({ page }) => {
    const addFilesButton = page.locator('[data-testid="add-files-button"]')
    await expect(addFilesButton).toBeVisible()
  })

  test('should display start conversion button', async ({ page }) => {
    const startButton = page.locator('[data-testid="start-conversion-button"]')
    await expect(startButton).toBeVisible()
  })
})
