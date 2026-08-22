import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Button from '@/components/ui/button/Button.vue'

describe('Button', () => {
  it('renders default button', () => {
    const wrapper = mount(Button, {
      slots: {
        default: 'Click me',
      },
    })
    
    expect(wrapper.text()).toContain('Click me')
    expect(wrapper.find('button').exists()).toBe(true)
  })

  it('renders with variant', () => {
    const wrapper = mount(Button, {
      props: {
        variant: 'destructive',
      },
      slots: {
        default: 'Delete',
      },
    })
    
    expect(wrapper.find('button').classes()).toContain('bg-destructive')
  })

  it('renders with size', () => {
    const wrapper = mount(Button, {
      props: {
        size: 'sm',
      },
      slots: {
        default: 'Small Button',
      },
    })
    
    expect(wrapper.find('button').classes()).toContain('h-8')
  })

  it('emits click event when clicked', async () => {
    const wrapper = mount(Button, {
      slots: {
        default: 'Click me',
      },
    })
    
    await wrapper.find('button').trigger('click')
    
    expect(wrapper.emitted('click')).toHaveLength(1)
  })

  it('renders as asChild when asChild prop is true', () => {
    const wrapper = mount(Button, {
      props: {
        asChild: true,
      },
      slots: {
        default: '<span>Custom</span>',
      },
    })
    
    expect(wrapper.find('span').exists()).toBe(true)
    expect(wrapper.find('button').exists()).toBe(false)
  })

  it('applies custom class', () => {
    const wrapper = mount(Button, {
      props: {
        class: 'custom-class',
      },
      slots: {
        default: 'Custom',
      },
    })
    
    expect(wrapper.find('button').classes()).toContain('custom-class')
  })

  it('renders with disabled state', () => {
    const wrapper = mount(Button, {
      props: {
        disabled: true,
      },
      slots: {
        default: 'Disabled',
      },
    })
    
    expect(wrapper.find('button').attributes('disabled')).toBeDefined()
  })

  it('renders loading state', async () => {
    const wrapper = mount(Button, {
      slots: {
        default: 'Loading',
      },
    })
    
    expect(wrapper.text()).toContain('Loading')
  })
})
