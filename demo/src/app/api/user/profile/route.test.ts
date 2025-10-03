import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { NextRequest } from 'next/server'
import { GET } from './route'

// Mock Clerk auth
vi.mock('@clerk/nextjs/server', () => ({
  auth: vi.fn(),
}))

// Mock fetch for MCP server calls
const mockFetch = vi.fn()
global.fetch = mockFetch

describe('/api/user/profile', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('returns 401 when user is not authenticated', async () => {
    const { auth } = await import('@clerk/nextjs/server')
    vi.mocked(auth).mockResolvedValue({ getToken: null })

    const response = await GET()
    expect(response.status).toBe(401)

    const data = await response.json()
    expect(data.error).toBe('Unauthorized')
  })

  it('successfully proxies request to MCP server', async () => {
    const { auth } = await import('@clerk/nextjs/server')
    const mockToken = 'mock-session-token'

    vi.mocked(auth).mockResolvedValue({
      getToken: vi.fn().mockResolvedValue(mockToken)
    })

    const mockProfileResponse = {
      ok: true,
      data: {
        id: 'user-123',
        email: 'test@example.com',
        tier: 'pro',
        createdAt: '2025-01-01T00:00:00Z'
      }
    }

    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => mockProfileResponse
    })

    const response = await GET()
    expect(response.status).toBe(200)

    const data = await response.json()
    expect(data).toEqual(mockProfileResponse)

    // Verify MCP server was called with correct headers
    expect(mockFetch).toHaveBeenCalledWith(
      'http://localhost:7145/user/profile',
      expect.objectContaining({
        headers: expect.objectContaining({
          'Authorization': `Bearer ${mockToken}`,
          'Content-Type': 'application/json',
        })
      })
    )
  })

  it('handles MCP server errors gracefully', async () => {
    const { auth } = await import('@clerk/nextjs/server')
    vi.mocked(auth).mockResolvedValue({
      getToken: vi.fn().mockResolvedValue('mock-token')
    })

    mockFetch.mockResolvedValue({
      ok: false,
      status: 500,
      json: async () => ({ message: 'Internal server error' })
    })

    const response = await GET()
    expect(response.status).toBe(500)

    const data = await response.json()
    expect(data.error).toBe('Failed to fetch profile')
  })

  it('handles network errors', async () => {
    const { auth } = await import('@clerk/nextjs/server')
    vi.mocked(auth).mockResolvedValue({
      getToken: vi.fn().mockResolvedValue('mock-token')
    })

    mockFetch.mockRejectedValue(new Error('Network error'))

    const response = await GET()
    expect(response.status).toBe(500)

    const data = await response.json()
    expect(data.error).toBe('Internal server error')
  })

  it('uses custom MCP server URL when configured', async () => {
    // Mock environment variable
    process.env.MCP_SERVER_URL = 'http://custom-mcp-server:8080'

    const { auth } = await import('@clerk/nextjs/server')
    vi.mocked(auth).mockResolvedValue({
      getToken: vi.fn().mockResolvedValue('mock-token')
    })

    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => ({ ok: true, data: {} })
    })

    await GET()

    expect(mockFetch).toHaveBeenCalledWith(
      'http://custom-mcp-server:8080/user/profile',
      expect.any(Object)
    )

    // Reset environment
    delete process.env.MCP_SERVER_URL
  })
})
