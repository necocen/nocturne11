import { useState } from "react";

// ブラウザのストレージに値を保存するhook
// cf. https://usehooks.com/useLocalStorage/
function useWebStorage<T>(key: string, initialValue: T, storage: Storage): [T, (value: T | ((prevState: T) => T)) => void] {
    // State to store our value
    // Pass initial state function to useState so logic is only executed once
    const [storedValue, setStoredValue] = useState<T>(() => {
        try {
            // Get from local storage by key
            const item = storage.getItem(key);
            // Parse stored json or if none return initialValue
            return item ? JSON.parse(item) : initialValue;
        } catch (error) {
            // If error also return initialValue
            console.error(error);
            return initialValue;
        }
    });

    // Return a wrapped version of useState's setter function that ...
    // ... persists the new value to localStorage.
    const setValue = (value: T | ((prevState: T) => T)) => {
        try {
            // Allow value to be a function so we have same API as useState
            const valueToStore = value instanceof Function ? value(storedValue) : value;
            // Save state
            setStoredValue(valueToStore);
            // Save to local storage
            if (valueToStore == undefined) {
                storage.removeItem(key);
            } else {
                storage.setItem(key, JSON.stringify(valueToStore));
            }
        } catch (error) {
            // A more advanced implementation would handle the error case
            console.error(error);
        }
    };

    return [storedValue, setValue];
}

// ブラウザのセッションストレージに値を保存するhook
export function useSessionStorage<T>(key: string, initialValue: T): [T, (value: T | ((prevState: T) => T)) => void] {
    return useWebStorage(key, initialValue, window.sessionStorage);
}

// ブラウザのローカルストレージに値を保存するhook
export function useLocalStorage<T>(key: string, initialValue: T): [T, (value: T | ((prevState: T) => T)) => void] {
    return useWebStorage(key, initialValue, window.localStorage);
}
