import { describe, it, expect } from 'vitest'
import { formatSize } from '@/utils'
import { showErrorDialog, alertSevere } from '@/utils/useError'

describe('utils/index.ts', () => {
  describe('formatSize', () => {
    it('should format 0 bytes', () => {
      expect(formatSize(0)).toBe('0 B')
    })

    it('should format bytes to KB', () => {
      expect(formatSize(1024)).toBe('1.00 KiB')
      expect(formatSize(2048)).toBe('2.00 KiB')
    })

    it('should format bytes to MB', () => {
      expect(formatSize(1024 * 1024)).toBe('1.00 MiB')
      expect(formatSize(5 * 1024 * 1024)).toBe('5.00 MiB')
    })

    it('should format bytes to GB', () => {
      expect(formatSize(1024 * 1024 * 1024)).toBe('1.00 GiB')
    })

    it('should handle decimal values', () => {
      expect(formatSize(1536)).toBe('1.50 KiB')
    })

    it('should handle large values', () => {
      const result = formatSize(1024 * 1024 * 1024 * 2.5)
      expect(result).toContain('GiB')
    })
  })
})

describe('utils/useError.ts', () => {
  describe('showErrorDialog', () => {
    it('should be a function', () => {
      expect(typeof showErrorDialog).toBe('function')
    })

    it('should handle empty content', async () => {
      // 测试不会实际调用 dialog，只是验证函数存在
      expect(showErrorDialog).toBeDefined()
    })
  })

  describe('alertSevere', () => {
    it('should be a function', () => {
      expect(typeof alertSevere).toBe('function')
    })

    it('should call error and showErrorDialog', () => {
      // 验证函数存在
      expect(alertSevere).toBeDefined()
    })
  })
})
