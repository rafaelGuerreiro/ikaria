import { useEffect, useRef, useState, type ReactNode } from 'react';
import { useSpacetimeDB } from 'spacetimedb/react';
import { SubscriptionContext } from './useSubscriptionReady';

/**
 * Calls subscribeToAllTables() once when the connection becomes active
 * and exposes readiness via context for child components.
 */
export function SubscriptionProvider({ children }: { children: ReactNode }) {
  const { getConnection, isActive } = useSpacetimeDB();
  const [isReady, setIsReady] = useState(false);
  const subscribedRef = useRef(false);

  useEffect(() => {
    if (!isActive || subscribedRef.current) return;
    const connection = getConnection();
    if (!connection) return;

    subscribedRef.current = true;
    connection
      .subscriptionBuilder()
      .onApplied(() => setIsReady(true))
      .subscribeToAllTables();
  }, [isActive, getConnection]);

  return (
    <SubscriptionContext.Provider value={isReady}>{children}</SubscriptionContext.Provider>
  );
}
