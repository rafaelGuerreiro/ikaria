import { useEffect, useState } from 'react';
import { useSpacetimeDB } from 'spacetimedb/react';

type AnyTableDef = { accessorName?: string; table?: { accessorName: string } };

function getAccessorName(tableDef: AnyTableDef): string {
  if (tableDef.table) return tableDef.table.accessorName;
  if (tableDef.accessorName) return tableDef.accessorName;
  throw new Error('Cannot determine table accessor name');
}

/**
 * Reads rows from a SpacetimeDB table in the local client cache.
 * Does NOT create a per-table subscription — use with SubscriptionProvider
 * which calls subscribeToAllTables() once for the whole connection.
 */
/*
export function useTable<TableDef extends UntypedTableDef>(
  query: Query<TableDef>,
  callbacks?: UseTableCallbacks<Prettify<RowType<TableDef>>>
): [readonly Prettify<RowType<TableDef>>[], boolean] {
*/
export function useLocalTable<T>(tableDef: AnyTableDef): T[] {
  const { getConnection, isActive } = useSpacetimeDB();
  const accessorName = getAccessorName(tableDef);
  const [rows, setRows] = useState<T[]>([]);

  useEffect(() => {
    const connection = getConnection();
    if (!connection) return;

    const table = (connection.db as Record<string, unknown>)[accessorName] as {
      iter(): Iterable<T>;
      onInsert(cb: unknown): void;
      onDelete(cb: unknown): void;
      onUpdate?: (cb: unknown) => void;
      removeOnInsert(cb: unknown): void;
      removeOnDelete(cb: unknown): void;
      removeOnUpdate?: (cb: unknown) => void;
    };

    const refresh = () => setRows(Array.from(table.iter()));
    refresh();

    table.onInsert(refresh);
    table.onDelete(refresh);
    table.onUpdate?.(refresh);
    return () => {
      table.removeOnInsert(refresh);
      table.removeOnDelete(refresh);
      table.removeOnUpdate?.(refresh);
    };
  }, [getConnection, accessorName, isActive]);

  return rows;
}
