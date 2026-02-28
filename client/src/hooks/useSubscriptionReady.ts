import { createContext, useContext } from 'react';

export const SubscriptionContext = createContext(false);

/**
 * Returns true once subscribeToAllTables() has been applied and the
 * local client cache is populated with initial data.
 */
export function useSubscriptionReady(): boolean {
  return useContext(SubscriptionContext);
}
