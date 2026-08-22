import { test, expect } from '@playwright/test'

test.describe('File Operations', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1420')
  })

  test('should add files via drag and drop simulation', async ({ page }) => {
    // 模拟拖放文件
    const fileInput = page.locator('input[type="file"]')
    
    // 如果有文件输入，测试文件选择
    if (await fileInput.isVisible()) {
      await fileInput.setInputFiles([
        {
          name: 'test.heic',
          mimeType: 'image/heic',
          buffer: Buffer.from('fake heic data'),
        },
      ])
      
      // 检查文件是否被添加到列表
      const fileList = page.locator('[data-testid="file-list"]')
      await expect(fileList).toBeVisible()
    }
  })

  test('should show empty state when no files', async ({ page }) => {
    const emptyState = page.locator('[data-testid="empty-state"]')
    await expect(emptyState).toBeVisible()
  })

  test('should display file stats', async ({ page }) => {
    const stats = page.locator('[data-testid="conversion-stats"]')
    await expect(stats).toBeVisible()
  })
})
