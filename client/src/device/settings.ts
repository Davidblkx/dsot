import { Settings } from '$infra/settings.ts';

export interface DeviceSettings {
    interactionMode: 'unset' | 'touch' | 'mouse';
    debugMode: boolean;
}

export const deviceSettings = new Settings<DeviceSettings>({
    key: 'dsot.device.settings',
    defaultValue: {
        interactionMode: 'unset',
        debugMode: false,
    },
});

export function debugAction<TArgs extends unknown[], TResult>(
    action: (...args: TArgs) => TResult,
    message: string,
    ...args: unknown[]
): (...args: TArgs) => TResult {
    return (...actionArgs: TArgs): TResult => {
        if (deviceSettings.get('debugMode')) {
            console.debug(message, ...args);
        }
        return action(...actionArgs);
    };
}

export function debugLog(message: string, ...args: unknown[]) {
    if (deviceSettings.get('debugMode')) {
        console.debug(message, ...args);
    }
}
