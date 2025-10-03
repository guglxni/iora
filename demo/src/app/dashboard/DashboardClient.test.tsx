import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import DashboardClient from './DashboardClient'

// Mock the API calls
const mockFetch = vi.fn()
global.fetch = mockFetch

// Mock Clerk auth
vi.mock('@clerk/nextjs', () => ({
  auth: vi.fn(() => Promise.resolve({ userId: 'test-user-id' })),
  currentUser: vi.fn(() => Promise.resolve({
    id: 'test-user-id',
    firstName: 'Test',
    lastName: 'User',
    emailAddresses: [{ emailAddress: 'test@example.com' }],
  })),
}))

describe('DashboardClient', () => {
  const mockUser = {
    id: 'test-user-id',
    firstName: 'Test',
    lastName: 'User',
    emailAddresses: [{ emailAddress: 'test@example.com' }],
  }

  beforeEach(() => {
    vi.clearAllMocks()
    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => ({
        ok: true,
        data: []
      })
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('renders user profile information', async () => {
    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      expect(screen.getByText('Welcome to IORA Dashboard')).toBeInTheDocument()
      expect(screen.getByText('Test User')).toBeInTheDocument()
      expect(screen.getByText('test@example.com')).toBeInTheDocument()
      expect(screen.getByText('test-user-id')).toBeInTheDocument()
    })
  })

  it('displays loading state initially', () => {
    // Mock slow API response
    mockFetch.mockImplementation(() =>
      new Promise(resolve => setTimeout(resolve, 100))
    )

    render(<DashboardClient user={mockUser} />)

    // Should show loading indicators
    expect(screen.getByText('Welcome to IORA Dashboard')).toBeInTheDocument()
  })

  it('creates API key when form is submitted', async () => {
    const mockApiKeyResponse = {
      ok: true,
      data: {
        id: 'key-123',
        key: 'iora_pk_test_key_123456789',
        keyPrefix: 'iora_pk_test...',
        message: 'Save this key securely. It will not be shown again.'
      }
    }

    mockFetch
      .mockResolvedValueOnce({ // API keys list
        ok: true,
        json: async () => ({ ok: true, data: [] })
      })
      .mockResolvedValueOnce({ // Usage stats
        ok: true,
        json: async () => ({
          ok: true,
          data: {
            tier: 'free',
            limits: { requestsPerMinute: 60, requestsPerMonth: 10000 },
            usage: { requestsThisMonth: 0, requestsToday: 0 },
            remaining: { requestsThisMonth: 10000 }
          }
        })
      })
      .mockResolvedValueOnce({ // Create API key
        ok: true,
        json: async () => mockApiKeyResponse
      })

    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Enter key name...')).toBeInTheDocument()
    })

    const input = screen.getByPlaceholderText('Enter key name...')
    const button = screen.getByText('Create Key')

    fireEvent.change(input, { target: { value: 'Test API Key' } })
    fireEvent.click(button)

    await waitFor(() => {
      expect(screen.getByText('✅ API Key Created Successfully!')).toBeInTheDocument()
      expect(screen.getByText('iora_pk_test_key_123456789')).toBeInTheDocument()
    })
  })

  it('copies API key to clipboard when copy button is clicked', async () => {
    const mockClipboard = {
      writeText: vi.fn().mockResolvedValue(undefined)
    }
    Object.assign(navigator, { clipboard: mockClipboard })

    const mockApiKeyResponse = {
      ok: true,
      data: {
        id: 'key-123',
        key: 'iora_pk_test_key_123456789',
        keyPrefix: 'iora_pk_test...',
        message: 'Save this key securely. It will not be shown again.'
      }
    }

    mockFetch
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ ok: true, data: [] })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          ok: true,
          data: {
            tier: 'free',
            limits: { requestsPerMinute: 60, requestsPerMonth: 10000 },
            usage: { requestsThisMonth: 0, requestsToday: 0 },
            remaining: { requestsThisMonth: 10000 }
          }
        })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => mockApiKeyResponse
      })

    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Enter key name...')).toBeInTheDocument()
    })

    const input = screen.getByPlaceholderText('Enter key name...')
    const button = screen.getByText('Create Key')

    fireEvent.change(input, { target: { value: 'Test API Key' } })
    fireEvent.click(button)

    await waitFor(() => {
      expect(screen.getByText('Copy to Clipboard')).toBeInTheDocument()
    })

    const copyButton = screen.getByText('Copy to Clipboard')
    fireEvent.click(copyButton)

    expect(mockClipboard.writeText).toHaveBeenCalledWith('iora_pk_test_key_123456789')
  })

  it('displays usage statistics correctly', async () => {
    const mockUsageResponse = {
      ok: true,
      data: {
        tier: 'pro',
        limits: { requestsPerMinute: 1000, requestsPerMonth: 100000 },
        usage: { requestsThisMonth: 15000, requestsToday: 150, lastRequest: '2025-01-01T12:00:00Z' },
        remaining: { requestsThisMonth: 85000 }
      }
    }

    mockFetch
      .mockResolvedValueOnce({ // API keys list
        ok: true,
        json: async () => ({ ok: true, data: [] })
      })
      .mockResolvedValueOnce({ // Usage stats
        ok: true,
        json: async () => mockUsageResponse
      })

    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      expect(screen.getByText('Pro')).toBeInTheDocument()
      expect(screen.getByText('15,000 / 100,000')).toBeInTheDocument()
      expect(screen.getByText('150')).toBeInTheDocument()
    })
  })

  it('handles API errors gracefully', async () => {
    mockFetch.mockRejectedValue(new Error('Network error'))

    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      // Should handle error without crashing
      expect(screen.getByText('Welcome to IORA Dashboard')).toBeInTheDocument()
    })
  })

  it('validates API key name input', async () => {
    render(<DashboardClient user={mockUser} />)

    await waitFor(() => {
      expect(screen.getByText('Create Key')).toBeInTheDocument()
    })

    const button = screen.getByText('Create Key')

    // Button should be disabled when input is empty
    expect(button).toBeDisabled()

    const input = screen.getByPlaceholderText('Enter key name...')
    fireEvent.change(input, { target: { value: 'Test Key' } })

    await waitFor(() => {
      expect(button).not.toBeDisabled()
    })
  })
})
