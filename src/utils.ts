import { clsx, ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge'

export function tw(...style: ClassValue[]) {
    return twMerge(clsx(style));
}