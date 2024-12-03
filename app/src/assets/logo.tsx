import React from 'react'

const Logo = (props: React.SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="1.151 -0.338 78 78"
    {...props}
  >
    <defs>
      <linearGradient id="d" x1={0} x2={0} y1={1} y2={0}>
        <stop offset={0} stopColor="#ff0844" />
        <stop offset={1} stopColor="#ffb199" />
      </linearGradient>
      <filter id="a" filterUnits="userSpaceOnUse">
        <feColorMatrix values="0 0 0 0 0.99609375 0 0 0 0 0.99609375 0 0 0 0 0.99609375 0 0 0 1 0" />
      </filter>
    </defs>
    <mask id="b">
      <g filter="url(#a)">
        <path
          fill="#333"
          fillOpacity={0}
          strokeWidth={2}
          d="M0 0h66v59.265H0z"
        />
        <svg
          width={66}
          height={59.265}
          style={{
            overflow: 'visible'
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="-0.03 0 111.403 100.035"
          >
            <path
              fill="#333"
              fillRule="evenodd"
              d="M67.062 6.58a13.193 13.193 0 1 0-22.83 13.23l24.21 42a13.203 13.203 0 0 0 22.84-13.25l-24.22-42zM1.732 93.42a13.2 13.2 0 1 0 4.81-18 13.2 13.2 0 0 0-4.81 18zm85 0a13.2 13.2 0 1 0 4.81-18 13.21 13.21 0 0 0-4.81 18zm-40.9-50 21.23 36.77a13.193 13.193 0 1 1-22.83 13.23l-21.25-36.81a13.2 13.2 0 1 1 22.84-13.22z"
            />
          </svg>
        </svg>
        <defs>
          <filter id="c">
            <feColorMatrix values="0 0 0 0 0.98046875 0 0 0 0 0.73046875 0 0 0 0 0.06640625 0 0 0 1 0" />
          </filter>
        </defs>
      </g>
    </mask>
    <g fill="#333">
      <g mask="url(#b)" transform="translate(7.15 5.662)">
        <path fill="none" d="M0 0h66v59.265H0z" />
        <svg
          width={66}
          height={59.265}
          filter="url(#c)"
          style={{
            overflow: 'visible'
          }}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="-0.03 0 111.403 100.035"
          >
            <path
              fillRule="evenodd"
              d="M67.062 6.58a13.193 13.193 0 1 0-22.83 13.23l24.21 42a13.203 13.203 0 0 0 22.84-13.25l-24.22-42zM1.732 93.42a13.2 13.2 0 1 0 4.81-18 13.2 13.2 0 0 0-4.81 18zm85 0a13.2 13.2 0 1 0 4.81-18 13.21 13.21 0 0 0-4.81 18zm-40.9-50 21.23 36.77a13.193 13.193 0 1 1-22.83 13.23l-21.25-36.81a13.2 13.2 0 1 1 22.84-13.22z"
            />
          </svg>
        </svg>
        <path
          fill="url(#d)"
          d="M0 0h78v71.265H0z"
          style={{
            pointerEvents: 'none'
          }}
          transform="translate(-6 -6)"
        />
      </g>
    </g>
  </svg>
)
export default Logo
