import { describe, it, expect } from 'vitest'
import { cn } from '@/lib/utils'

describe('utils', () => {
  describe('cn', () => {
    it('merges class names correctly', () => {
      const result = cn('px-4', 'py-2', 'bg-blue-500')
      expect(result).toBe('px-4 py-2 bg-blue-500')
    })

    it('handles conditional classes', () => {
      const result = cn('px-4', {
        'py-2': true,
        'bg-blue-500': false,
      })
      expect(result).toBe('px-4 py-2')
    })

    it('removes duplicate classes', () => {
      const result = cn('px-4', 'px-4', 'py-2')
      expect(result).toBe('px-4 py-2')
    })

    it('handles empty input', () => {
      const result = cn()
      expect(result).toBe('')
    })

    it('handles mixed input types', () => {
      const result = cn('px-4', null, undefined, 'py-2', false && 'bg-red-500')
      expect(result).toBe('px-4 py-2')
    })

    it('handles arrays', () => {
      const result = cn(['px-4', 'py-2'])
      expect(result).toBe('px-4 py-2')
    })
  })
})
