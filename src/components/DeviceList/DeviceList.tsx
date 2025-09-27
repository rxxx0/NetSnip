import React, { useState, useMemo } from "react";
import { Grid, List } from "lucide-react";
import { DeviceCard } from "../DeviceCard/DeviceCard";
import { FilterDropdown } from "../common/FilterDropdown";
import { useNetworkStore } from "../../stores/networkStore";
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from '@dnd-kit/core';
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
  rectSortingStrategy,
} from '@dnd-kit/sortable';

export const DeviceList: React.FC = () => {
  const { scanning, getFilteredDevices, searchQuery, filter, setFilter } = useNetworkStore();
  const [viewMode, setViewMode] = useState<"grid" | "list">("grid");

  // Get filtered devices based on search and status filter
  const searchFilteredDevices = getFilteredDevices();

  // Load saved order from localStorage
  const loadSavedOrder = () => {
    const savedOrder = localStorage.getItem('deviceOrder');
    if (savedOrder) {
      try {
        const orderIds = JSON.parse(savedOrder) as string[];
        // Sort devices according to saved order
        const orderedDevices = [...searchFilteredDevices].sort((a, b) => {
          const aIndex = orderIds.indexOf(a.id);
          const bIndex = orderIds.indexOf(b.id);
          // If neither is in saved order, maintain original order
          if (aIndex === -1 && bIndex === -1) return 0;
          // If only one is in saved order, put it first
          if (aIndex === -1) return 1;
          if (bIndex === -1) return -1;
          // Both are in saved order, sort by saved index
          return aIndex - bIndex;
        });
        return orderedDevices;
      } catch (e) {
        return searchFilteredDevices;
      }
    }
    return searchFilteredDevices;
  };

  const [orderedDevices, setOrderedDevices] = useState(loadSavedOrder());

  React.useEffect(() => {
    // Update orderedDevices when searchFilteredDevices changes
    // This ensures the grid updates when device status changes
    setOrderedDevices(prevOrdered => {
      const savedOrder = localStorage.getItem('deviceOrder');
      let orderIds: string[] = [];

      if (savedOrder) {
        try {
          orderIds = JSON.parse(savedOrder);
        } catch (e) {
          orderIds = [];
        }
      }

      // Sort based on saved order
      const newOrder = [...searchFilteredDevices].sort((a, b) => {
        const aIndex = orderIds.indexOf(a.id);
        const bIndex = orderIds.indexOf(b.id);

        if (aIndex === -1 && bIndex === -1) {
          // Neither in saved order, check if they were in previous order
          const prevAIndex = prevOrdered.findIndex(d => d.id === a.id);
          const prevBIndex = prevOrdered.findIndex(d => d.id === b.id);
          if (prevAIndex !== -1 && prevBIndex !== -1) {
            return prevAIndex - prevBIndex;
          }
          return 0;
        }
        if (aIndex === -1) return 1;
        if (bIndex === -1) return -1;
        return aIndex - bIndex;
      });

      return newOrder;
    });
  }, [searchFilteredDevices]);

  const filteredDevices = useMemo(() => {
    return orderedDevices.filter((device) => {
      if (filter === "all") return true;
      return device.status === filter;
    });
  }, [orderedDevices, filter]);

  // Setup drag and drop sensors with activation constraints
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 5, // Minimum drag distance before activation
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (active.id !== over?.id) {
      const oldIndex = filteredDevices.findIndex((d) => d.id === active.id);
      const newIndex = filteredDevices.findIndex((d) => d.id === over?.id);

      if (oldIndex !== -1 && newIndex !== -1) {
        const newOrder = arrayMove(filteredDevices, oldIndex, newIndex);
        setOrderedDevices(newOrder);

        // Save the new order to localStorage
        const orderIds = newOrder.map(device => device.id);
        localStorage.setItem('deviceOrder', JSON.stringify(orderIds));
      }
    }
  };

  return (
    <div className="neu-card rounded-2xl p-6 animate-fade-in">
      {/* Header */}
      <div className="flex items-end justify-between mb-4">
        <h2 className="text-2xl font-semibold text-text-primary">
          Connected Devices
        </h2>

        <div className="flex items-center gap-3">
          {/* Filter Dropdown */}
          <FilterDropdown value={filter} onChange={setFilter} />

          {/* View Mode Toggle */}
          <div className="flex gap-1 neu-card p-1 rounded-lg">
            <button
              onClick={() => setViewMode("grid")}
              className={`p-2 rounded-lg transition-all ${
                viewMode === "grid"
                  ? "neu-button"
                  : "hover:bg-gray-100 dark:hover:bg-gray-800"
              }`}
              aria-label="Grid view"
            >
              <Grid className="w-4 h-4" />
            </button>
            <button
              onClick={() => setViewMode("list")}
              className={`p-2 rounded-lg transition-all ${
                viewMode === "list"
                  ? "neu-button"
                  : "hover:bg-gray-100 dark:hover:bg-gray-800"
              }`}
              aria-label="List view"
            >
              <List className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Loading State */}
      {scanning && (
        <div className="flex flex-col items-center justify-center py-20 animate-fade-in">
          <div className="spinner"></div>
          <p className="mt-4 text-text-secondary animate-slide-up">
            Scanning network...
          </p>
        </div>
      )}

      {/* No Devices State */}
      {!scanning && filteredDevices.length === 0 && (
        <div className="flex flex-col items-center justify-center py-20 animate-fade-in">
          <div className="w-24 h-24 rounded-2xl neu-pressed flex items-center justify-center mb-4 animate-scale-in">
            <div
              className="w-12 h-12 rounded-lg"
              style={{
                background: `linear-gradient(145deg, var(--text-muted), var(--text-secondary))`,
              }}
            />
          </div>
          <p className="text-lg font-medium text-text-primary animate-slide-up">
            No devices found
          </p>
          <p
            className="text-sm text-text-secondary mt-1 animate-slide-up"
            style={{ animationDelay: "100ms" }}
          >
            {searchQuery
              ? "Try a different search term"
              : filter !== "all"
                ? "Try changing the filter"
                : "Make sure you're connected to a network"}
          </p>
        </div>
      )}

      {/* Device Grid/List */}
      {!scanning && filteredDevices.length > 0 && (
        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          onDragEnd={handleDragEnd}
        >
          <SortableContext
            items={filteredDevices.map(d => d.id)}
            strategy={viewMode === "grid" ? rectSortingStrategy : verticalListSortingStrategy}
          >
            <div
              className={
                viewMode === "grid"
                  ? "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6 transition-all duration-300"
                  : "space-y-4 transition-all duration-300"
              }
            >
              {filteredDevices.map((device) => (
                <DeviceCard key={device.id} device={device} viewMode={viewMode} />
              ))}
            </div>
          </SortableContext>
        </DndContext>
      )}
    </div>
  );
};
